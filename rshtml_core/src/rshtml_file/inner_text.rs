use super::{Input, component::component_tag_identifier};
use proc_macro2::TokenStream;
use quote::quote;
use winnow::{
    ModalResult, Parser,
    ascii::multispace0,
    combinator::{alt, not, repeat},
    token::{any, take_while},
};

pub fn inner_text<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
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
    .map(|text| (text.chars().count(), quote! { write!(out, "{}", #text)?; }))
    .parse_next(input)?;

    input.state.text_size += text_size;

    Ok(text_ts)
}
