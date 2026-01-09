use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::quote;
use syn::parse2;
use winnow::ModalResult;
use winnow::combinator::{alt, cut_err, eof, opt, repeat, terminated};
use winnow::error::{ContextError, ErrMode, StrContext, StrContextValue};
use winnow::{Parser, token::any};

pub fn compile(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut tokens = tokens.as_slice();

    let r#move = if let Some(TokenTree::Ident(ident)) = tokens.first() {
        if ident == "move" {
            tokens = &tokens[1..];
            quote! {move }
        } else {
            quote! {}
        }
    } else {
        quote! {}
    };

    let body = terminated(template, eof.context(StrContext::Label("end of template")))
        // template
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
        any.verify(|tt: &TokenTree| match tt {
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => false,
            TokenTree::Punct(p) if p.as_char() == '<' => false,
            _ => true,
        }),
    )
    .fold(String::new, |mut acc, item: TokenTree| {
        acc.push_str(&item.to_string());
        acc.push(' ');
        acc
    })
    .map(|s| quote! { write!(f, "{}", #s)?; })
    .parse_next(input)
}

fn tag(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    let open_tag = (lt, ident, attributes, cut_err(gt))
        .map(|(lt, ident, attributes, gt)| {
            let mut ts = TokenStream::new();

            let starting = format!(" {lt}{ident}");
            ts.extend(quote! { write!(f, "{}", #starting)?; });
            ts.extend(attributes);
            let gt = format!(" {gt}");
            ts.extend(quote! { write!(f, "{}", #gt)?; });

            ts
        })
        .context(StrContext::Label("tag open"));

    let close_tag = (lt, slash, cut_err(ident), cut_err(gt))
        .map(|(lt, slash, ident, gt)| {
            let closing = format!(" {lt}{slash}{ident}{gt}");
            quote! { write!(f, "{}", #closing)?; }
        })
        .context(StrContext::Label("tag close"));

    let self_close_tag = (lt, ident, attributes, slash, cut_err(gt))
        .map(|(lt, ident, attributes, slash, gt)| {
            let mut ts = TokenStream::new();

            let starting = format!(" {lt}{ident}");
            ts.extend(quote! { write!(f, "{}", #starting)?; });
            ts.extend(attributes);
            let closing = format!(" {slash}{gt}");
            ts.extend(quote! { write!(f, "{}", #closing)?; });

            ts
        })
        .context(StrContext::Label("self-closing tag"));

    alt((
        self_close_tag,
        (open_tag, template, cut_err(close_tag)).map(|(open_tag, template, close_tag)| {
            let mut ts = TokenStream::new();

            ts.extend(open_tag);
            ts.extend(template);
            ts.extend(close_tag);

            ts
        }),
    ))
    .parse_next(input)
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
                let attr = format!(" {name}");
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
