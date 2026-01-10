use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::quote;
use syn::parse2;
use winnow::ModalResult;
use winnow::combinator::{alt, cut_err, eof, fail, not, opt, peek, preceded, repeat, terminated};
use winnow::error::{ContextError, ErrMode, StrContext, StrContextValue};
use winnow::stream::Stream;
use winnow::{Parser, token::any};

pub fn compile(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut tokens = tokens.as_slice();

    let r#move = if let Some(TokenTree::Ident(ident)) = tokens.first() {
        if ident == "move" {
            tokens = &tokens[1..];
            quote! {move }
        } else {
            TokenStream::new()
        }
    } else {
        TokenStream::new()
    };

    let body = terminated(template, eof.context(StrContext::Label("end of template")))
        .parse_next(&mut tokens)
        .unwrap_or_else(|e: ErrMode<ContextError>| {
            let span = tokens
                .first()
                .map(|tt| tt.span())
                .unwrap_or_else(Span::call_site);

            let err = e.into_inner().unwrap();
            let msg = err
                .context()
                .filter_map(|c| match c {
                    StrContext::Label(l) => Some(l.to_string()),
                    StrContext::Expected(e) => Some(format!("expected {}", e)),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join(": ");

            let msg = format!("compile error: {msg}");
            let msg_lit = syn::LitStr::new(&msg, span);

            quote::quote_spanned! { span =>
                compile_error!(#msg_lit);
            }
        });

    quote! {
        ::rshtml::ViewFn(#r#move |f: &mut dyn std::fmt::Write| -> std::fmt::Result {
            #body
            Ok(())
        })
    }
    .into()
}

fn template(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    repeat(0.., alt((expr, tag, text)))
        .fold(TokenStream::new, |mut acc, tt: TokenStream| {
            acc.extend(tt);
            acc
        })
        .parse_next(input)
}

fn expr(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    let group: Group = any
        .verify_map(|tt: TokenTree| match tt {
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => Some(g),
            _ => None,
        })
        .parse_next(input)?;

    let stream = group.stream();

    let output = if let Ok(expr) = parse2::<syn::Expr>(stream.clone()) {
        quote! { ::rshtml::Exp(&(#expr)).render(f)?; }
    } else if let Ok(block) = parse2::<syn::Block>(stream.clone()) {
        quote! { ::rshtml::Exp({#block}).render(f)?; }
    } else {
        quote! { ::rshtml::Exp({#stream}).render(f)?; }
    };

    Ok(output)
}

fn text(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    repeat(
        1..,
        alt((
            html_entity,
            any.verify(|tt: &TokenTree| match tt {
                TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => false,
                TokenTree::Punct(p) if p.as_char() == '<' => false,
                _ => true,
            })
            .map(|tt: TokenTree| tt.to_string()),
        )),
    )
    .fold(String::new, |mut acc, item| {
        acc.push(' ');
        acc.push_str(&item);
        acc
    })
    .map(|mut s| {
        s.push(' ');
        quote! { write!(f, "{}", #s)?; }
    })
    .parse_next(input)
}

fn tag(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    let mut open_tag = (lt, ident, attributes, cut_err(gt))
        .map(|(lt, ident, attributes, gt)| {
            let mut ts = TokenStream::new();

            if attributes.is_empty() {
                let starting = format!(" {lt}{ident}{gt} ");
                ts.extend(quote! { write!(f, "{}", #starting)?; });
            } else {
                let starting = format!(" {lt}{ident}");
                ts.extend(quote! { write!(f, "{}", #starting)?; });
                ts.extend(attributes);
                let gt = format!("{gt} ");
                ts.extend(quote! { write!(f, "{}", #gt)?; });
            }

            (ts, ident)
        })
        .context(StrContext::Label("tag open"));

    let close_tag = (lt, slash, cut_err(ident), cut_err(gt))
        .map(|(lt, slash, ident, gt)| {
            let closing = format!(" {lt}{slash}{ident}{gt} ");
            let ts = quote! { write!(f, "{}", #closing)?; };

            (ts, ident)
        })
        .context(StrContext::Label("tag close"));

    let mut self_close_tag = (lt, ident, attributes, slash, cut_err(gt))
        .map(|(lt, ident, attributes, slash, gt)| {
            let mut ts = TokenStream::new();

            if attributes.is_empty() {
                let starting = format!(" {lt}{ident}{slash}{gt} ");
                ts.extend(quote! { write!(f, "{}", #starting)?; });
            } else {
                let starting = format!(" {lt}{ident}");
                ts.extend(quote! { write!(f, "{}", #starting)?; });
                ts.extend(attributes);
                let closing = format!("{slash}{gt} ");
                ts.extend(quote! { write!(f, "{}", #closing)?; });
            }
            ts
        })
        .context(StrContext::Label("self-closing tag"));

    // ----------------

    let checkpoint = input.checkpoint();

    match self_close_tag.parse_next(input) {
        Ok(ts) => return Ok(ts),
        Err(ErrMode::Backtrack(_)) => {
            input.reset(&checkpoint);
        }
        Err(e) => return Err(e),
    }

    let open_checkpoint = input.checkpoint();
    let (open_ts, open_tag_name) = open_tag.parse_next(input)?;

    let body_ts = match open_tag_name.to_string().as_str() {
        "script" => script_tag_body.parse_next(input)?,
        "style" => style_tag_body.parse_next(input)?,
        _ => template.parse_next(input)?,
    };

    let close_checkpoint = input.checkpoint();
    let close_opt = opt(close_tag).parse_next(input)?;

    match close_opt {
        Some((close_ts, close_tag_name)) => {
            if open_tag_name.to_string() != close_tag_name.to_string() {
                input.reset(&close_checkpoint);

                let expected_str = format!("corresponding closing tag for <{}>", open_tag_name);
                let exp_static: &'static str = Box::leak(expected_str.into_boxed_str());

                return cut_err(fail)
                    .context(StrContext::Expected(StrContextValue::Description(
                        exp_static,
                    )))
                    .parse_next(input);
            }
            let mut ts = TokenStream::new();
            ts.extend(open_ts);
            ts.extend(body_ts);
            ts.extend(close_ts);
            Ok(ts)
        }
        None => {
            input.reset(&open_checkpoint);

            cut_err(fail)
                .context(StrContext::Label("tag opened here but never closed"))
                .context(StrContext::Expected(StrContextValue::Description(
                    "matching closing tag",
                )))
                .parse_next(input)
        }
    }
}

fn attributes(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    repeat(0.., attribute)
        .fold(TokenStream::new, |mut acc, item| {
            acc.extend(item);
            acc
        })
        .parse_next(input)
}

fn attribute(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    let name = repeat(1.., alt((ident, hyphen))).fold(TokenStream::new, |mut acc, i| {
        acc.extend(i);
        acc
    });

    (name, opt((equal, alt((expr, string_literal)))))
        .map(|(name, equal_value)| {
            let mut ts = TokenStream::new();

            if let Some((equal, value)) = equal_value {
                let attr = format!(" {name}{equal}");
                ts.extend(quote! { write!(f, "{}", #attr)?; });
                ts.extend(quote! {#value });
            } else {
                let attr = format!(" {name} ");
                ts.extend(quote! { write!(f, "{}", #attr)?; });
            }

            ts
        })
        .parse_next(input)
}

fn string_literal(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Literal(lit) => {
            let s = lit.to_string();
            Some(quote! { write!(f, "{}", #s)?; })
        }
        _ => None,
    })
    .parse_next(input)
}

fn script_tag_body(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    let script_end_tag = (
        lt,
        slash,
        any.verify(|tt: &TokenTree| matches!(tt, TokenTree::Ident(i) if i.to_string() == "script")),
        gt,
    );

    repeat(0.., preceded(not(peek(script_end_tag)), any))
        .fold(TokenStream::new, |mut acc, tt| {
            acc.extend(std::iter::once(tt));
            acc
        })
        .map(|ts| {
            let ts = ts.to_string();
            quote! { write!(f, "{}", #ts)?; }
        })
        .parse_next(input)
}

fn style_tag_body(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    let style_end_tag = (
        lt,
        slash,
        any.verify(|tt: &TokenTree| matches!(tt, TokenTree::Ident(i) if i.to_string() == "style")),
        gt,
    );

    repeat(0.., preceded(not(peek(style_end_tag)), any))
        .fold(TokenStream::new, |mut acc, tt| {
            acc.extend(std::iter::once(tt));
            acc
        })
        .map(|ts| {
            let ts = ts.to_string();
            quote! { write!(f, "{}", #ts)?; }
        })
        .parse_next(input)
}

fn html_entity(input: &mut &[TokenTree]) -> ModalResult<String> {
    let amp = any.verify(|tt: &TokenTree| matches!(tt, TokenTree::Punct(p) if p.as_char() == '&'));

    let body = alt((
        (
            any.verify(|tt: &TokenTree| matches!(tt, TokenTree::Punct(p) if p.as_char() == '#')),
            any.verify(|tt: &TokenTree| match tt {
                // HEX CONTROL (x1F600)
                TokenTree::Ident(i) => {
                    let s = i.to_string();
                    if s.starts_with('x') || s.starts_with('X') {
                        s[1..].chars().all(|c| c.is_ascii_hexdigit())
                    } else {
                        false
                    }
                }
                // DECIMAL CONTROL (123)
                TokenTree::Literal(l) => l.to_string().chars().all(|c| c.is_ascii_digit()),
                _ => false,
            }),
        )
            .map(|(_, val)| format!("#{}", val)),
        // B. Name Entity (copy, nbsp)
        any.verify_map(|tt: TokenTree| match tt {
            TokenTree::Ident(i) => Some(i.to_string()),
            _ => None,
        }),
    ));

    let semi =
        opt(any.verify(|tt: &TokenTree| matches!(tt, TokenTree::Punct(p) if p.as_char() == ';')));

    (amp, body, semi)
        .map(|(_, body, semi)| {
            let mut s = String::from("&");
            s.push_str(&body);
            if semi.is_some() {
                s.push(';');
            }

            s
        })
        .parse_next(input)
}

// TOKENS

fn ident(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Ident(i) => Some(quote! {#i}),
        _ => None,
    })
    .context(StrContext::Expected(StrContextValue::StringLiteral(
        "ident",
    )))
    .parse_next(input)
}

// fn ident_with_span(input: &mut &[TokenTree]) -> ModalResult<(TokenStream, Span)> {
//     any.verify_map(|tt: TokenTree| match tt {
//         TokenTree::Ident(i) => Some((quote! {#i}, i.span())),
//         _ => None,
//     })
//     .context(StrContext::Label("ident"))
//     .parse_next(input)
// }

fn lt(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Punct(p) if p.as_char() == '<' => Some(quote! {#p}),
        _ => None,
    })
    .context(StrContext::Expected(StrContextValue::CharLiteral('<')))
    .parse_next(input)
}

fn gt(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Punct(p) if p.as_char() == '>' => Some(quote! {#p}),
        _ => None,
    })
    .context(StrContext::Expected(StrContextValue::CharLiteral('>')))
    .parse_next(input)
}

// fn not_gt(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
//     any.verify_map(|tt: TokenTree| match tt {
//         TokenTree::Punct(p) if p.as_char() == '>' => None,
//         _ => Some(quote! {#tt}),
//     })
//     .context(StrContext::Label("not gt"))
//     .parse_next(input)
// }

fn equal(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Punct(p) if p.as_char() == '=' => Some(quote! {#p}),
        _ => None,
    })
    .context(StrContext::Expected(StrContextValue::CharLiteral('=')))
    .parse_next(input)
}

fn hyphen(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Punct(p) if p.as_char() == '-' => Some(quote! {#p}),
        _ => None,
    })
    .context(StrContext::Expected(StrContextValue::CharLiteral('-')))
    .parse_next(input)
}

// fn not_hyphen(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
//     any.verify_map(|tt: TokenTree| match tt {
//         TokenTree::Punct(p) if p.as_char() == '-' => None,
//         _ => Some(quote! {#tt}),
//     })
//     .context(StrContext::Label("not hyphen"))
//     .parse_next(input)
// }

fn slash(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Punct(p) if p.as_char() == '/' => Some(quote! {#p}),
        _ => None,
    })
    .context(StrContext::Expected(StrContextValue::CharLiteral('/')))
    .parse_next(input)
}

// fn exclamation(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
//     any.verify_map(|tt: TokenTree| match tt {
//         TokenTree::Punct(p) if p.as_char() == '!' => Some(quote! {#p}),
//         _ => None,
//     })
//     .context(StrContext::Label("exclamation mark"))
//     .parse_next(input)
// }

// fn backslash(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
//     any.verify_map(|tt: TokenTree| match tt {
//         TokenTree::Punct(p) if p.as_char() == '\\' => Some(quote! {#p}),
//         _ => None,
//     })
//     .context(StrContext::Label("Token Error Comes From Me"))
//     .parse_next(input)
// }

// fn dq(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
//     any.verify_map(|tt: TokenTree| match tt {
//         TokenTree::Punct(p) if p.as_char() == '"' => Some(quote! {#p}),
//         _ => None,
//     })
//     .context(StrContext::Label("Token Error Comes From Me"))
//     .parse_next(input)
// }

// fn sq(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
//     any.verify_map(|tt: TokenTree| match tt {
//         TokenTree::Punct(p) if p.as_char() == '\'' => Some(quote! {#p}),
//         _ => None,
//     })
//     .context(StrContext::Label("Token Error Comes From Me"))
//     .parse_next(input)
// }
