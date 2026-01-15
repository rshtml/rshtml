use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::parse2;
use winnow::ModalResult;
use winnow::combinator::{alt, cut_err, eof, fail, opt, peek, repeat, terminated};
use winnow::error::{StrContext, StrContextValue};
use winnow::stream::Stream;
use winnow::{Parser, token::any};

// TODO: Enable file reading using the v_file!  or vfile! macro.

enum Node {
    Expr(TokenStream),
    Text(String),
}

pub fn compile(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut tokens = tokens.as_slice();

    let (expr_defs, body, text_size) =
        match terminated(template, eof_with_check).parse_next(&mut tokens) {
            Ok((expr_defs, nodes)) => {
                let mut body = TokenStream::new();
                let mut text_buffer = String::new();
                let mut first = true;
                let mut text_size = 0;

                for node in nodes {
                    match node {
                        Node::Expr(tokens) => {
                            if !text_buffer.is_empty() {
                                text_buffer.push(' ');
                                body.extend(quote! { write!(out, "{}", #text_buffer)?; });
                                text_buffer.clear();
                            }

                            body.extend(tokens);
                        }
                        Node::Text(text) => {
                            if !first {
                                text_buffer.push(' ');
                            }
                            text_buffer.push_str(&text);
                            text_size += text.len();
                            first = false;
                        }
                    }
                }

                if !text_buffer.is_empty() {
                    body.extend(quote! { write!(out, "{}", #text_buffer)?; });
                }

                (expr_defs, body, text_size)
            }
            Err(e) => {
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
                move |out: &mut dyn std::fmt::Write| -> std::fmt::Result {
                    #body
                    Ok(())
                },
                _text_size
            )
        })
    }
}

fn eof_with_check<'a>(input: &mut &'a [TokenTree]) -> ModalResult<&'a [TokenTree]> {
    if !input.is_empty() && peek((lt, slash)).parse_next(input).is_ok() {
        return cut_err(fail)
            .context(StrContext::Label("tag closed here but never opened"))
            .parse_next(input);
    }

    eof.context(StrContext::Label("end of template"))
        .parse_next(input)
}

fn template(input: &mut &[TokenTree]) -> ModalResult<(TokenStream, Vec<Node>)> {
    repeat(
        0..,
        alt((
            expr.map(|(ts, expr)| (ts, vec![Node::Expr(expr)])),
            tag,
            text.map(|t| (TokenStream::new(), vec![Node::Text(t)])),
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
            quote! { ::rshtml::Exp(&(#def_ident)).render(out)?; },
        )
    } else if let Ok(block) = parse2::<syn::Block>(stream.clone()) {
        (
            quote! { let #def_ident = {#block}; _text_size += ::rshtml::TextSize(&#def_ident).text_size(); },
            quote! { ::rshtml::Exp(&(#def_ident)).render(out)?; },
        )
    } else {
        (
            quote! { let #def_ident = {#stream}; _text_size += ::rshtml::TextSize(&#def_ident).text_size(); },
            quote! { ::rshtml::Exp(&(#def_ident)).render(out)?; },
        )
    };

    Ok(output)
}

fn text(input: &mut &[TokenTree]) -> ModalResult<String> {
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
        if !acc.is_empty() {
            acc.push(' ');
        }
        acc.push_str(&item);
        acc
    })
    .parse_next(input)
}

fn tag(input: &mut &[TokenTree]) -> ModalResult<(TokenStream, Vec<Node>)> {
    alt((self_closing_tag, full_tag)).parse_next(input)
}

fn full_tag(input: &mut &[TokenTree]) -> ModalResult<(TokenStream, Vec<Node>)> {
    let open_checkpoint = input.checkpoint();

    let (open_expr_defs, open_nodes, open_ident) = (lt, ident, attributes, cut_err(gt))
        .map(|(lt, ident, (expr_defs, attributes), gt)| {
            let mut nodes = Vec::new();

            if attributes.is_empty() {
                nodes.push(Node::Text(format!("{lt}{ident}{gt}")));
            } else {
                nodes.push(Node::Text(format!("{lt}{ident}")));
                nodes.extend(attributes);
                nodes.push(Node::Text(format!("{gt}")));
            }

            (expr_defs, nodes, ident)
        })
        .context(StrContext::Label("tag open"))
        .parse_next(input)?;

    let (body_expr_defs, body_nodes) = template.parse_next(input)?;

    let close_checkpoint = input.checkpoint();
    let Some((close_nodes, close_ident)) = opt((lt, slash, cut_err(ident), cut_err(gt))
        .map(|(lt, slash, ident, gt)| (Node::Text(format!("{lt}{slash}{ident}{gt}")), ident))
        .context(StrContext::Label("tag close")))
    .parse_next(input)?
    else {
        input.reset(&open_checkpoint);
        return cut_err(fail)
            .context(StrContext::Label("tag opened here but never closed"))
            .context(StrContext::Expected(StrContextValue::Description(
                "matching closing tag",
            )))
            .parse_next(input);
    };

    if open_ident.to_string() != close_ident.to_string() {
        input.reset(&close_checkpoint);
        let expected_str = format!("corresponding closing tag for <{}>", open_ident);
        let exp_static: &'static str = Box::leak(expected_str.into_boxed_str());

        return cut_err(fail)
            .context(StrContext::Expected(StrContextValue::Description(
                exp_static,
            )))
            .parse_next(input);
    }

    let mut expr_defs = TokenStream::new();
    expr_defs.extend(open_expr_defs);
    expr_defs.extend(body_expr_defs);

    let mut nodes = Vec::new();
    nodes.extend(open_nodes);
    nodes.extend(body_nodes);
    nodes.push(close_nodes);

    Ok((expr_defs, nodes))
}

fn self_closing_tag(input: &mut &[TokenTree]) -> ModalResult<(TokenStream, Vec<Node>)> {
    (lt, ident, attributes, slash, cut_err(gt))
        .map(|(lt, ident, (expr_defs, attributes), slash, gt)| {
            let mut nodes = Vec::new();

            if attributes.is_empty() {
                nodes.push(Node::Text(format!("{lt}{ident}{slash}{gt}")));
            } else {
                nodes.push(Node::Text(format!("{lt}{ident}")));
                nodes.extend(attributes);
                nodes.push(Node::Text(format!("{slash}{gt}")));
            }
            (expr_defs, nodes)
        })
        .context(StrContext::Label("self-closing tag"))
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
    let name = repeat(1.., alt((ident, hyphen))).fold(TokenStream::new, |mut acc, i| {
        acc.extend(i);
        acc
    });

    (
        name,
        opt((
            equal,
            alt((
                expr.map(|(expr_def, expr)| (expr_def, Node::Expr(expr))),
                string_literal.map(|sl| (TokenStream::new(), Node::Text(sl))),
            )),
        )),
    )
        .map(|(name, equal_value)| {
            let mut expr_defs = TokenStream::new();

            if let Some((equal, (expr_def, value))) = equal_value {
                expr_defs.extend(expr_def);

                (expr_defs, format!("{name}{equal}"), Some(value))
            } else {
                (expr_defs, format!("{name}"), None)
            }
        })
        .parse_next(input)
}

fn string_literal(input: &mut &[TokenTree]) -> ModalResult<String> {
    any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Literal(lit) => Some(lit.to_string()),
        _ => None,
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
                        let hex = &s[1..];
                        (1..=6).contains(&hex.len()) && hex.chars().all(|c| c.is_ascii_hexdigit())
                    } else {
                        false
                    }
                }
                // DECIMAL CONTROL (123)
                TokenTree::Literal(l) => {
                    let s = l.to_string();

                    (1..=5).contains(&s.len()) && s.chars().all(|c| c.is_ascii_digit())
                }
                _ => false,
            }),
        )
            .map(|(_, val)| format!("#{}", val)),
        // Name Entity (copy, nbsp)
        any.verify_map(|tt: TokenTree| match tt {
            TokenTree::Ident(i) => {
                let s = i.to_string();

                if (1..=30).contains(&s.len()) && s.chars().all(|c| c.is_ascii_alphabetic()) {
                    Some(s)
                } else {
                    None
                }
            }
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

// fn exclamation(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
//     any.verify_map(|tt: TokenTree| match tt {
//         TokenTree::Punct(p) if p.as_char() == '!' => Some(quote! {#p}),
//         _ => None,
//     })
//     .context(StrContext::Label("exclamation mark"))
//     .parse_next(input)
// }
