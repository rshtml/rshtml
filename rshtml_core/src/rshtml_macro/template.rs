use crate::rshtml_macro::{
    Input, extensions::ParserDiagnostic, rust_block::rust_block,
    simple_expr_paren::simple_expr_paren, template_params::template_params, text::text,
    use_directive::use_directive,
};
use proc_macro2::TokenStream;
use quote::quote;
use winnow::{
    ModalResult, Parser,
    ascii::multispace0,
    combinator::{alt, eof, not, opt, peek, repeat},
    token::{any, none_of, take_while},
};

pub fn template<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    (
        opt("\u{FEFF}").void(),
        opt((
            peek(('@', multispace0, '(')),
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
        alt((
            text.label("html text"),
            (
                '@',
                multispace0,
                alt((
                    use_directive.label("use directive"),
                    rust_block.label("code block"),
                    child_content_directive.label("child content"),
                    continue_directive.label("continue"),
                    break_directive.label("break"),
                    simple_expr_paren.label("parenthesized expression"),
                )),
            )
                .map(|(_, _, ts)| ts),
        )),
    )
    .fold(TokenStream::new, |mut acc, ts| {
        acc.extend(ts);
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

pub fn continue_directive<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    ("continue", multispace0)
        .map(|(_, _)| quote! { continue })
        .parse_next(input)
}

pub fn break_directive<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    ("break", multispace0)
        .map(|(_, _)| quote! { break })
        .parse_next(input)
}

pub fn child_content_directive<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    (
        "child_content",
        alt(("()".void(), peek(not(rust_identifier)).void())),
    )
        .map(|(_, _)| quote! {child_content(__f__)?;})
        .parse_next(input)
}

pub fn escape_or_raw(expr_ts: TokenStream, is_escaped: bool, message: &str) -> TokenStream {
    if is_escaped {
        quote! { ::rshtml::Expr(&(#expr_ts)).render(&mut ::rshtml::EscapingWriter { inner: __f__ }, #message)?; }
    } else {
        quote! { ::rshtml::Expr(&(#expr_ts)).render(__f__, #message)?; }
    }
}
