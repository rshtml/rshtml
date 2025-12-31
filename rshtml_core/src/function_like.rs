use chumsky::prelude::*;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::quote;
use syn::parse2;

pub fn compile(input: TokenStream) -> TokenStream {
    let iter = input.into_iter();
    let tokens = iter.collect::<Vec<TokenTree>>();

    match template().parse(&tokens).into_result() {
        Ok(result) => result,
        Err(errors) => {
            panic!("Parse error: {:?}", errors);
        }
    }
}

trait Parsed<'a>: Parser<'a, &'a [TokenTree], TokenStream, extra::Err<Simple<'a, TokenTree>>> {}
impl<'a, T> Parsed<'a> for T where
    T: Parser<'a, &'a [TokenTree], TokenStream, extra::Err<Simple<'a, TokenTree>>>
{
}

fn template<'a>() -> impl Parsed<'a> {
    choice((text(), expr()))
        .repeated()
        .collect::<Vec<TokenStream>>()
        .map(|streams| streams.into_iter().collect::<TokenStream>())
}

fn at<'a>() -> impl Parsed<'a> {
    any()
        .filter(|t| matches!(t, TokenTree::Punct(p) if p.as_char() == '@'))
        .to(quote! {})
}

fn text<'a>() -> impl Parsed<'a> {
    // any().filter(|t| !matches!(t, TokenTree::Punct(p) if p.as_char() == '@'))
    any()
        .and_is(at().not())
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|tokens| {
            let mut s = String::new();
            for t in tokens {
                s.push_str(&t.to_string());
                s.push(' ');
            }
            quote! { write!(buffer, "{} ", #s).unwrap(); }
        })
}

fn expr<'a>() -> impl Parsed<'a> {
    at().ignore_then(select! {
        TokenTree::Group(g) if matches!(g.delimiter(), Delimiter::Brace | Delimiter::Parenthesis) => g
    })
    .map(|group| {
        let stream = group.stream();
        match parse2::<syn::Expr>(stream.clone()) {
            Ok(expr) => quote! { write!(buffer, "{}", #expr).unwrap(); },
            Err(_) => quote! { write!(buffer, "{}", #stream).unwrap(); },
        }
    })
}
