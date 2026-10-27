use crate::extensions::ParserDiagnostic;
use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::parse2;
use winnow::ModalResult;
use winnow::combinator::{alt, cut_err, eof, fail, opt, repeat, terminated};
use winnow::error::{StrContext, StrContextValue};
use winnow::{Parser, token::any};

enum Node {
    Expr(TokenStream),
    Text(String),
}

pub fn compile(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut tokens = tokens.as_slice();

    let (expr_defs, body, text_size) = match terminated(
        template,
        eof.expected("string literal (\"..\"), expression ({..}), or HTML tag (<..>)"),
    )
    .parse_next(&mut tokens)
    {
        Ok((expr_defs, nodes)) => {
            let mut body = TokenStream::new();
            let mut text_buffer = String::new();
            let mut text_size = 0;

            for node in nodes {
                match node {
                    Node::Expr(tokens) => {
                        if !text_buffer.is_empty() {
                            body.extend(quote! { out.write_str(#text_buffer)?; });
                            text_buffer.clear();
                        }

                        body.extend(tokens);
                    }
                    Node::Text(text) => {
                        text_buffer.push_str(&text);
                        text_size += text.len();
                    }
                }
            }

            if !text_buffer.is_empty() {
                body.extend(quote! { out.write_str(#text_buffer)?; });
            }

            (expr_defs, body, text_size)
        }
        Err(e) => {
            let span = tokens.first().map(|tt| tt.span()).unwrap_or_else(Span::call_site);

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

            (
                TokenStream::new(),
                quote::quote_spanned! { span =>
                    compile_error!(#msg_lit);
                },
                0,
            )
        }
    };

    quote! {
        ::rshtml::ViewFn::new({
            let mut _text_size = #text_size;
            #expr_defs

            (
                move |out: &mut dyn ::rshtml::Write| -> std::fmt::Result {
                    #body
                    Ok(())
                },
                _text_size
            )
        })
    }
}

fn template(input: &mut &[TokenTree]) -> ModalResult<(TokenStream, Vec<Node>)> {
    repeat(
        0..,
        alt((
            expr.map(|(ts, expr)| (ts, vec![Node::Expr(expr)])),
            text.map(|t| (TokenStream::new(), vec![Node::Text(t)])),
            tag,
        )),
    )
    .fold(
        || (TokenStream::new(), Vec::new()),
        |(mut expr_defs, mut bodies), (expr_def, body)| {
            expr_defs.extend(expr_def);
            bodies.extend(body);
            (expr_defs, bodies)
        },
    )
    .parse_next(input)
}

fn expr(input: &mut &[TokenTree]) -> ModalResult<(TokenStream, TokenStream)> {
    let group: Group = any
        .verify_map(|tt: TokenTree| match tt {
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => Some(g),
            _ => None,
        })
        .parse_next(input)?;

    let stream = group.stream();

    let def_ident = format_ident!("_exp{}", input.len());

    let output = if let Ok(expr) = parse2::<syn::Expr>(stream.clone()) {
        (
            quote! { let #def_ident = (#expr); _text_size += ::rshtml::TextSize(&#def_ident).text_size(); },
            quote! { ::rshtml::Exp(&(#def_ident)).render(&mut ::rshtml::EscapingWriter{ inner: out })?; },
        )
    } else if let Ok(block) = parse2::<syn::Block>(stream.clone()) {
        (
            quote! { let #def_ident = {#block}; _text_size += ::rshtml::TextSize(&#def_ident).text_size(); },
            quote! { ::rshtml::Exp(&(#def_ident)).render(&mut ::rshtml::EscapingWriter{ inner: out })?; },
        )
    } else {
        (
            quote! { let #def_ident = {#stream}; _text_size += ::rshtml::TextSize(&#def_ident).text_size(); },
            quote! { ::rshtml::Exp(&(#def_ident)).render(&mut ::rshtml::EscapingWriter{ inner: out })?; },
        )
    };

    Ok(output)
}

fn text(input: &mut &[TokenTree]) -> ModalResult<String> {
    any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Literal(lit) => syn::parse2::<syn::LitStr>(quote!(#lit)).ok().map(|s| s.value()),
        _ => None,
    })
    .parse_next(input)
}

fn tag(input: &mut &[TokenTree]) -> ModalResult<(TokenStream, Vec<Node>)> {
    let close_tag =
        (slash, cut_err((tag_name, gt)).label("unclosed tag")).map(|(slash, (tag_name, gt))| {
            (TokenStream::new(), vec![Node::Text(format!("<{slash}{tag_name}{gt}"))])
        });

    let open_or_self_close_tag = (
        tag_name,
        attributes,
        cut_err(alt((
            (slash, cut_err(gt).label("unclosed tag")).map(|(slash, gt)| format!("{slash}{gt}")),
            gt.map(|gt| gt.to_string()),
            fail.expected("> or />").label("unclosed tag"),
        ))),
    )
        .map(|(tag_name, (expr_defs, attributes), close)| {
            let mut nodes = Vec::new();

            if attributes.is_empty() {
                nodes.push(Node::Text(format!("<{tag_name}{close}")));
            } else {
                nodes.push(Node::Text(format!("<{tag_name}")));
                nodes.extend(attributes);
                nodes.push(Node::Text(close.to_string()));
            }
            (expr_defs, nodes)
        });

    let doctype = (
        exclamation,
        cut_err((
            ident.verify(|i| i.to_string().to_lowercase() == "doctype").expected("DOCTYPE"),
            ident.verify(|i| i.to_string().to_lowercase() == "html").expected("html"),
            gt,
        ))
        .label("invalid <!DOCTYPE html>"),
    )
        .map(|(exclamation, (ident, ident2, gt))| {
            (TokenStream::new(), vec![Node::Text(format!("<{exclamation}{ident} {ident2}{gt}"))])
        });

    (lt, cut_err(alt((open_or_self_close_tag, close_tag, doctype, fail.expected("tag name")))))
        .map(|(_, res)| res)
        .parse_next(input)
}

fn tag_name(input: &mut &[TokenTree]) -> ModalResult<String> {
    let tag_ident_start = ident.verify_map(|i| {
        let i = i.to_string();
        let mut chars = i.chars();

        if let Some(first) = chars.next()
            && first.is_ascii_alphabetic()
            && chars.all(|c| c.is_ascii_alphanumeric())
        {
            Some(i)
        } else {
            None
        }
    });

    let tag_ident = ident
        .verify(|i| i.to_string().chars().all(|c| c.is_ascii_alphanumeric()))
        .map(|i| i.to_string());

    let number = any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Literal(l) => {
            let s = l.to_string();
            if s.chars().next().is_some_and(|c| c.is_ascii_digit()) { Some(s) } else { None }
        }
        _ => None,
    });

    let non_alpha = repeat(1.., alt((hyphen.map(|h| h.to_string()), number))).fold(
        String::new,
        |mut acc, item| {
            acc.push_str(&item);
            acc
        },
    );

    let rest = repeat(0.., (non_alpha, tag_ident)).fold(String::new, |mut acc, item| {
        acc.push_str(&item.0);
        acc.push_str(&item.1);
        acc
    });

    (tag_ident_start, rest)
        .map(|(name, rest)| format!("{}{}", name, rest).trim().to_string())
        .parse_next(input)
}

fn attributes(input: &mut &[TokenTree]) -> ModalResult<(TokenStream, Vec<Node>)> {
    repeat(0.., attribute)
        .fold(
            || (TokenStream::new(), Vec::new()),
            |(mut expr_defs, mut items), (expr_def, name, value)| {
                expr_defs.extend(expr_def);

                items.push(Node::Text(name));
                if let Some(val) = value {
                    items.push(val);
                }
                (expr_defs, items)
            },
        )
        .parse_next(input)
}

fn attribute(input: &mut &[TokenTree]) -> ModalResult<(TokenStream, String, Option<Node>)> {
    (
        attribute_name,
        opt((
            equal,
            cut_err(alt((
                expr.map(|(expr_def, expr)| {
                    let quoted_expr = quote! {
                        out.write_str("\"")?;
                        #expr
                        out.write_str("\"")?;
                    };

                    (expr_def, Node::Expr(quoted_expr))
                }),
                attribute_value.map(|attr_val| (TokenStream::new(), Node::Text(attr_val))),
            )))
            .expected("attribute value"),
        )),
    )
        .map(|(attribute_name, equal_value)| {
            let mut expr_defs = TokenStream::new();

            if let Some((equal, (expr_def, value))) = equal_value {
                expr_defs.extend(expr_def);

                (expr_defs, format!(" {attribute_name}{equal}"), Some(value))
            } else {
                (expr_defs, format!(" {attribute_name}"), None)
            }
        })
        .parse_next(input)
}

fn attribute_name(input: &mut &[TokenTree]) -> ModalResult<String> {
    let mut prev_end: Option<proc_macro2::LineColumn> = None;

    repeat(
        1..,
        any.verify(|tt: &TokenTree| {
            if let Some(end) = prev_end
                && end != tt.span().start()
            {
                return false;
            }

            let is_valid = match tt {
                TokenTree::Punct(p) if "<=>/".contains(p.as_char()) => false,
                TokenTree::Literal(l) => {
                    let s = l.to_string();
                    !s.starts_with('"') && !s.starts_with('\'')
                }
                TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => false,
                _ => true,
            };

            if is_valid {
                prev_end = Some(tt.span().end());
            }

            is_valid
        }),
    )
    .fold(String::new, |mut acc, tt: TokenTree| {
        acc.push_str(&tt.to_string());
        acc
    })
    .parse_next(input)
}

fn attribute_value(input: &mut &[TokenTree]) -> ModalResult<String> {
    any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Literal(lit) => {
            let s = lit.to_string();
            if s.starts_with('"')
                || s.starts_with('\'')
                || s.chars().next().is_some_and(|c| c.is_ascii_digit())
            {
                Some(s)
            } else {
                None
            }
        }
        TokenTree::Ident(ident) => Some(ident.to_string()),
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
    .context(StrContext::Expected(StrContextValue::StringLiteral("identifier")))
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

fn slash(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Punct(p) if p.as_char() == '/' => Some(quote! {#p}),
        _ => None,
    })
    .context(StrContext::Expected(StrContextValue::CharLiteral('/')))
    .parse_next(input)
}

fn exclamation(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Punct(p) if p.as_char() == '!' => Some(quote! {#p}),
        _ => None,
    })
    .context(StrContext::Expected(StrContextValue::CharLiteral('!')))
    .parse_next(input)
}
