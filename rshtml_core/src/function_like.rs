use chumsky::prelude::*;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::quote;
use syn::parse2;

pub fn compile(input: TokenStream) -> TokenStream {
    let iter = input.into_iter();
    let tokens = iter.collect::<Vec<TokenTree>>();

    // match template().parse(&tokens).into_result() {
    //     Ok(result) => result,
    //     Err(errors) => {
    //         panic!("Parse error: {:?}", errors);
    //     }
    // }

    let (r#move, tokens) = if let Some(TokenTree::Ident(ident)) = tokens.first() {
        if ident == "move" {
            (quote! {move }, &tokens[1..])
        } else {
            (quote! {}, tokens.as_slice())
        }
    } else {
        (quote! {}, tokens.as_slice())
    };

    let body = template()
        .parse(tokens)
        .into_result()
        .unwrap_or_else(|errors| {
            let errs = format!("Parse error: {:?}", errors);
            quote! { compile_error!("Parse error: {:?}", #errs); }
        });

    quote! {
        ::rshtml::ViewFn(#r#move |f: &mut dyn std::fmt::Write| -> std::fmt::Result {
            #body
            Ok(())
        })
    }
    .into()
}

trait Parsed<'a>:
    Parser<'a, &'a [TokenTree], TokenStream, extra::Err<Simple<'a, TokenTree>>> + Clone
{
}
impl<'a, T> Parsed<'a> for T where
    T: Parser<'a, &'a [TokenTree], TokenStream, extra::Err<Simple<'a, TokenTree>>> + Clone
{
}

fn template<'a>() -> impl Parsed<'a> {
    recursive(|node| {
        choice((expr(), tag(node), text()))
            .repeated()
            .collect::<Vec<TokenStream>>()
            .map(|streams| streams.into_iter().collect::<TokenStream>())
    })
}

fn expr<'a>() -> impl Parsed<'a> {
    select! { TokenTree::Group(g) if matches!(g.delimiter(), Delimiter::Brace) => g }.map(|group| {
        let stream = group.stream();
        match parse2::<syn::Block>(stream.clone()) {
            Ok(block) => quote! { ::rshtml::Exp({#block}).render(f)?; },
            Err(_) => quote! { ::rshtml::Exp({#stream}).render(f)?; },
        }
    })
}

fn text<'a>() -> impl Parsed<'a> {
    any()
        .filter(|t| {
            !matches!(t, TokenTree::Group(g) if g.delimiter() == Delimiter::Brace)
                && !matches!(t, TokenTree::Punct(p) if p.as_char() == '<')
        })
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|tokens| {
            let mut s = String::new();
            for t in tokens {
                s.push_str(&t.to_string());
                s.push(' ');
            }
            quote! { write!(f, "{} ", #s)?; }
        })
}

fn tag<'a>(node: impl Parsed<'a>) -> impl Parsed<'a> {
    let collect_tokens = |tokens: &[TokenTree]| {
        let mut s = String::new();
        for t in tokens {
            s.push_str(&t.to_string());
        }
        quote! { write!(f, "{} ", &(#s))?; }
    };

    let lt = any().filter(|t| matches!(t, TokenTree::Punct(p) if p.as_char() == '<'));
    let gt = any().filter(|t| matches!(t, TokenTree::Punct(p) if p.as_char() == '>'));
    let slash = any().filter(|t| matches!(t, TokenTree::Punct(p) if p.as_char() == '/'));
    let tag_name = select! { TokenTree::Ident(i) => i };

    let attributes = choice((
        expr(),
        any()
            .filter(
                |t| !matches!(t, TokenTree::Punct(p) if p.as_char() == '>' || p.as_char() == '/'),
            )
            .map(|t| {
                let s = t.to_string();
                quote! { write!(f, "{} ", &(#s))?; }
            }),
    ))
    .repeated()
    .collect::<Vec<TokenStream>>();

    let open_tag = lt
        .then(tag_name)
        .to_slice()
        .map(collect_tokens)
        .then(attributes);

    let self_closing = slash.then(gt).to_slice().map(collect_tokens);

    let close_tag = choice((
        self_closing,
        gt.then(node.or_not())
            .then(
                lt.then(slash)
                    .then(tag_name)
                    .then(gt)
                    .to_slice()
                    .map(collect_tokens),
            )
            .map(|((gt_token, content), close_tag_code)| {
                let gt_s = gt_token.to_string();
                quote! {
                    write!(f, "{}", &(#gt_s))?;
                    #content
                    #close_tag_code
                }
            }),
    ));

    open_tag
        .then(close_tag)
        .map(|((start_code, attrs), end_code)| {
            let attrs_code = attrs.into_iter().collect::<TokenStream>();
            quote! {
                #start_code
                #attrs_code
                #end_code
            }
        })
}
