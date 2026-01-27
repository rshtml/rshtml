use crate::rshtml_macro::{
    Input, extensions::ParserDiagnostic, rust_block::rust_block, template_params::template_params,
    text::text,
};
use proc_macro2::TokenStream;
use winnow::{
    ModalResult, Parser,
    combinator::{alt, eof, opt, peek, repeat},
    token::{any, take_while},
};

pub fn template<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    (
        opt("\u{FEFF}").void(),
        opt((
            peek(('@', '(')),
            template_params.label("template parameters"),
        ))
        .void(),
        template_content,
        eof.expected("end of file"),
    )
        .map(|(_, _, content, _)| content)
        .parse_next(input)
}

pub fn template_content<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    repeat(
        0..,
        alt((rust_block.label("code block"), text.label("html text"))),
    )
    .fold(TokenStream::new, |mut acc, txt| {
        acc.extend(txt);
        acc
    })
    .parse_next(input)
}

pub fn rust_identifier<'a>(input: &mut Input<'a>) -> ModalResult<&'a str> {
    take_while(1.., |c: char| c.is_alphanumeric() || c == '_')
        .verify(|s: &str| syn::parse_str::<syn::Ident>(s).is_ok())
        .parse_next(input)
}

pub fn component_tag_identifier<'a>(input: &mut Input<'a>) -> ModalResult<&'a str> {
    let start = input.input;

    (
        any.verify(|c: &char| c.is_ascii_uppercase()),
        take_while(0.., |c: char| c.is_ascii_alphanumeric()),
    )
        .parse_next(input)?;

    let consumed = start.len() - input.len();

    Ok(&start[..consumed])
}
