use super::{
    Input, component::component, inner_text::inner_text, rust_block::rust_block,
    rust_stmt::rust_stmt, simple_expr::simple_expr, simple_expr_paren::simple_expr_paren,
    template_params::template_params, text::text, use_directive::use_directive,
};
use crate::{
    extensions::ParserDiagnostic,
    rshtml_file::{
        break_directive::break_directive, child_content_directive::child_content_directive,
        continue_directive::continue_directive,
    },
};
use proc_macro2::TokenStream;
use quote::quote;
use winnow::{
    ModalResult, Parser,
    ascii::multispace0,
    combinator::{alt, cut_err, eof, fail, opt, peek, repeat},
    token::{any, none_of, one_of, take_while},
};

pub fn template<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    (
        opt("\u{FEFF}"),
        multispace0,
        opt((
            peek(('@', multispace0, '(')),
            template_params.label("template parameters"),
        ))
        .void(),
        template_content,
        eof.expected("end of file"),
    )
        .map(|(_, _, _, content, _)| content)
        .parse_next(input)
}

pub fn template_content<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    repeat(0.., alt((text.label("html text"), block)))
        .fold(TokenStream::new, |mut acc, ts| {
            acc.extend(ts);
            acc
        })
        .parse_next(input)
}

pub fn inner_template_content<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    (
        '{',
        repeat(0.., alt((inner_text.label("html text"), block))).fold(
            TokenStream::new,
            |mut acc, ts| {
                acc.extend(ts);
                acc
            },
        ),
        cut_err('}').expected("}"),
    )
        .map(|(_, content, _)| {
            let ts = quote! {{#content}};
            ts
        })
        .parse_next(input)
}

pub fn block<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    alt((
        component.label("component"),
        (
            '@',
            multispace0,
            alt((
                use_directive.label("use directive"),
                rust_block.label("code block"),
                rust_stmt.label("statement"),
                child_content_directive.label("child content"),
                continue_directive.label("continue"),
                break_directive.label("break"),
                simple_expr_paren.label("parenthesized expression"),
                simple_expr.label("expression"),
                cut_err(fail).expected("valid RsHtml expression"),
            )),
        )
            .map(|(_, _, ts)| ts),
        fail.expected("valid RsHtml expression"),
    ))
    .parse_next(input)
}

pub fn rust_identifier<'a>(input: &mut Input<'a>) -> ModalResult<&'a str> {
    let start = input.input;

    (
        one_of(|c: char| c.is_alphabetic() || c == '_'),
        take_while(0.., |c: char| c.is_alphanumeric() || c == '_'),
    )
        .parse_next(input)?;

    let consumed = start.len() - input.input.len();
    Ok(&start[..consumed])
}

pub fn string_line<'a>(input: &mut Input<'a>) -> ModalResult<&'a str> {
    let start = input.input;

    alt((
        (
            '"',
            repeat(0.., alt((("\\", any).void(), none_of(['"', '\\']).void())))
                .map(|_: Vec<()>| ()),
            '"',
        ),
        (
            '\'',
            repeat(0.., alt((("\\", any).void(), none_of(['\'', '\\']).void())))
                .map(|_: Vec<()>| ()),
            '\'',
        ),
    ))
    .void()
    .parse_next(input)?;

    let parsed_len = start.len() - input.input.len();
    Ok(&start[..parsed_len])
}
