use crate::rshtml_macro::{Input, template_params::template_params, text::text};
use proc_macro2::TokenStream;
use winnow::{
    ModalResult, Parser,
    combinator::{alt, not, opt, peek, repeat},
};

pub fn template<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    (
        opt("\u{FEFF}").void(),
        alt((
            (peek(('@', '(')), template_params).void(),
            not(('@', '(')).void(),
        ))
        .void(),
        template_content,
    )
        .map(|(_, _, content)| todo!())
        .parse_next(input)
}

pub fn template_content<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    repeat(0.., text)
        .fold(TokenStream::new, |mut acc, txt| {
            acc.extend(txt.1);
            acc
        })
        .parse_next(input)
}
