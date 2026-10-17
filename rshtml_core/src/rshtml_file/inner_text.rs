use super::{Input, component::component_tag_identifier};
use proc_macro2::TokenStream;
use quote::quote;
use winnow::{
    ModalResult, Parser,
    ascii::multispace0,
    combinator::{alt, not, repeat},
    token::{any, take_while},
};

#[cfg(not(debug_assertions))]
pub fn inner_text<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<TokenStream> {
    enum Chunk<'a> {
        Str(&'a str),
        Char(char),
    }

    let (text_size, text_ts) = repeat(
        1..,
        alt((
            take_while(1.., |c| c != '@' && c != '<' && c != '}').map(Chunk::Str),
            "@@".map(|_| Chunk::Char('@')),
            (
                not(alt((
                    '@'.void(),
                    '}'.void(),
                    ("<", multispace0, component_tag_identifier).void(),
                    ("</", multispace0, component_tag_identifier).void(),
                ))),
                any,
            )
                .map(|(_, c)| Chunk::Char(c)),
        )),
    )
    .fold(String::new, |mut acc, chunk| {
        match chunk {
            Chunk::Str(s) => acc.push_str(s),
            Chunk::Char(c) => acc.push(c),
        }
        acc
    })
    .map(|text| {
        (
            text.chars().count(),
            quote! { write!(__out__, "{}", #text)?; },
        )
    })
    .parse_next(input)?;

    input.state.info.text_size += text_size;

    Ok(text_ts)
}

#[cfg(debug_assertions)]
pub fn inner_text<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<TokenStream> {
    use crate::debug::Snippet;

    enum Chunk<'a> {
        Str(&'a str),
        Char(char),
    }

    let (text, span) = repeat(
        1..,
        alt((
            take_while(1.., |c| c != '@' && c != '<' && c != '}').map(Chunk::Str),
            "@@".map(|_| Chunk::Char('@')),
            (
                not(alt((
                    '@'.void(),
                    '}'.void(),
                    ("<", multispace0, component_tag_identifier).void(),
                    ("</", multispace0, component_tag_identifier).void(),
                ))),
                any,
            )
                .map(|(_, c)| Chunk::Char(c)),
        )),
    )
    .fold(String::new, |mut acc, chunk| {
        match chunk {
            Chunk::Str(s) => acc.push_str(s),
            Chunk::Char(c) => acc.push(c),
        }
        acc
    })
    .with_span()
    .parse_next(input)?;

    let text_size = text.len();
    let text_ts = quote! { write!(__out__, "{}", #text)?; };

    let bytes_to_delete = text_size - text.chars().count();
    input
        .state
        .info
        .debug
        .push(Snippet::Html(span, bytes_to_delete));

    input.state.info.text_size += text_size;

    Ok(text_ts)
}
