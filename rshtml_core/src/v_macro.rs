use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::parse2;
use winnow::ModalResult;
use winnow::combinator::{alt, eof, opt, repeat, repeat_till, terminated};
use winnow::error::{StrContext, StrContextValue};
use winnow::{Parser, token::any};

// TODO: Enable file reading using the v_file! macro.

enum Node {
    Expr(TokenStream),
    Text(String),
}

pub fn compile(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut tokens = tokens.as_slice();

    let (expr_defs, body, text_size) = match terminated(template, eof).parse_next(&mut tokens) {
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

fn template(input: &mut &[TokenTree]) -> ModalResult<(TokenStream, Vec<Node>)> {
    repeat(
        0..,
        alt((
            expr.map(|(ts, expr)| (ts, vec![Node::Expr(expr)])),
            group,
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

fn group(input: &mut &[TokenTree]) -> ModalResult<(TokenStream, Vec<Node>)> {
    let (tokens, (open, close)): (Vec<TokenTree>, (Node, Node)) = any
        .verify_map(|tt: TokenTree| match tt {
            TokenTree::Group(g)
                if matches!(g.delimiter(), Delimiter::Parenthesis | Delimiter::Bracket) =>
            {
                Some((
                    g.stream().into_iter().collect(),
                    match g.delimiter() {
                        Delimiter::Parenthesis => {
                            (Node::Text("(".to_owned()), Node::Text(")".to_owned()))
                        }
                        Delimiter::Bracket => {
                            (Node::Text("[".to_owned()), Node::Text("]".to_owned()))
                        }
                        _ => unreachable!(),
                    },
                ))
            }
            _ => None,
        })
        .parse_next(input)?;

    let mut tokens = tokens.as_slice();

    let (inner_expr_defs, inner_nodes) = template.parse_next(&mut tokens)?;

    let mut nodes = vec![open];
    nodes.extend(inner_nodes);
    nodes.push(close);

    Ok((inner_expr_defs, nodes))
}

fn text(input: &mut &[TokenTree]) -> ModalResult<String> {
    repeat(
        1..,
        alt((
            html_entity,
            any.verify(|tt: &TokenTree| match tt {
                TokenTree::Group(_) => false,
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
    let open_tag =
        (lt, tag_name, attributes, gt).map(|(lt, tag_name, (expr_defs, attributes), gt)| {
            let mut nodes = Vec::new();

            if attributes.is_empty() {
                nodes.push(Node::Text(format!("{lt}{tag_name}{gt}")));
            } else {
                nodes.push(Node::Text(format!("{lt}{tag_name}")));
                nodes.extend(attributes);
                nodes.push(Node::Text(format!("{gt}")));
            }

            (expr_defs, nodes)
        });

    let close_tag = (lt, slash, tag_name, gt).map(|(lt, slash, tag_name, gt)| {
        (
            TokenStream::new(),
            vec![Node::Text(format!("{lt}{slash}{tag_name}{gt}"))],
        )
    });

    let self_close_tag = (lt, tag_name, attributes, slash, gt).map(
        |(lt, tag_name, (expr_defs, attributes), slash, gt)| {
            let mut nodes = Vec::new();

            if attributes.is_empty() {
                nodes.push(Node::Text(format!("{lt}{tag_name}{slash}{gt}")));
            } else {
                nodes.push(Node::Text(format!("{lt}{tag_name}")));
                nodes.extend(attributes);
                nodes.push(Node::Text(format!("{slash}{gt}")));
            }
            (expr_defs, nodes)
        },
    );

    let doctype = (
        lt,
        exclamation,
        ident.verify(|i| i.to_string().to_lowercase() == "doctype"),
        ident.verify(|i| i.to_string().to_lowercase() == "html"),
        gt,
    )
        .map(|(lt, exclamation, ident, ident2, gt)| {
            (
                TokenStream::new(),
                vec![Node::Text(format!("{lt}{exclamation}{ident} {ident2}{gt}"))],
            )
        });

    alt((
        open_tag,
        close_tag,
        self_close_tag,
        doctype,
        html_comment,
        any.map(|tt: TokenTree| (TokenStream::new(), vec![Node::Text(tt.to_string())])),
    ))
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
            if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                Some(s)
            } else {
                None
            }
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
            alt((
                expr.map(|(expr_def, expr)| (expr_def, Node::Expr(expr))),
                attribute_value.map(|attr_val| (TokenStream::new(), Node::Text(attr_val))),
            )),
        )),
    )
        .map(|(attribute_name, equal_value)| {
            let mut expr_defs = TokenStream::new();

            if let Some((equal, (expr_def, value))) = equal_value {
                expr_defs.extend(expr_def);

                (expr_defs, format!("{attribute_name}{equal}"), Some(value))
            } else {
                (expr_defs, attribute_name.to_string(), None)
            }
        })
        .parse_next(input)
}

fn attribute_name(input: &mut &[TokenTree]) -> ModalResult<String> {
    repeat(
        1..,
        any.verify(|tt: &TokenTree| match tt {
            TokenTree::Punct(p) if "=>/".contains(p.as_char()) => false,
            TokenTree::Literal(l) => {
                let s = l.to_string();
                !s.starts_with('"') && !s.starts_with('\'')
            }
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => false,
            _ => true,
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

fn html_comment(input: &mut &[TokenTree]) -> ModalResult<(TokenStream, Vec<Node>)> {
    let comment_start =
        (lt, exclamation, hyphen, hyphen).map(|(lt, exclamation, hyphen, hyphen2)| {
            Node::Text(format!("{lt}{exclamation}{hyphen}{hyphen2}"))
        });

    let comment_end = (hyphen, hyphen, gt)
        .map(|(hyphen, hyphen2, gt)| Node::Text(format!("{hyphen}{hyphen2}{gt}")));

    let comment_body_with_end = repeat_till(
        0..,
        alt((
            expr.map(|(expr_def, expr)| (expr_def, vec![Node::Expr(expr)])),
            any.verify(
                |tt| !matches!(tt, TokenTree::Group(g) if g.delimiter() == Delimiter::Brace),
            )
            .map(|tt: TokenTree| (TokenStream::new(), vec![Node::Text(tt.to_string())])),
        )),
        comment_end,
    )
    .map(
        |(items, comment_end): (Vec<(TokenStream, Vec<Node>)>, Node)| {
            let mut all_nodes = Vec::new();
            let mut all_expr_defs = TokenStream::new();

            for (expr_defs, nodes) in items {
                all_expr_defs.extend(expr_defs);
                all_nodes.extend(nodes);
            }

            all_nodes.push(comment_end);

            (all_expr_defs, all_nodes)
        },
    );

    (comment_start, comment_body_with_end)
        .map(|(comment_start, (expr_defs, nodes))| {
            let mut all_nodes = Vec::with_capacity(nodes.len() + 1);
            all_nodes.push(comment_start);
            all_nodes.extend(nodes);

            (expr_defs, all_nodes)
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

fn exclamation(input: &mut &[TokenTree]) -> ModalResult<TokenStream> {
    any.verify_map(|tt: TokenTree| match tt {
        TokenTree::Punct(p) if p.as_char() == '!' => Some(quote! {#p}),
        _ => None,
    })
    .context(StrContext::Expected(StrContextValue::CharLiteral('!')))
    .parse_next(input)
}
