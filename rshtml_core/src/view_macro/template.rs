use crate::view_macro::{
    Input, component::component, extensions::ParserDiagnostic, inner_text::inner_text,
    rust_block::rust_block, rust_stmt::rust_stmt, simple_expr::simple_expr,
    simple_expr_paren::simple_expr_paren, template_params::template_params, text::text,
    use_directive::use_directive,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::{path::Path, str::FromStr};
use syn::Ident;
use winnow::{
    ModalResult, Parser,
    ascii::multispace0,
    combinator::{alt, cut_err, eof, not, opt, peek, repeat},
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
            )),
        )
            .map(|(_, _, ts)| ts),
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

pub fn extract_component_name(path: &Path) -> Option<String> {
    let filename = path.file_name().and_then(|n| n.to_str())?;
    let component_name = filename.strip_suffix(".rs.html").unwrap_or(filename);
    Some(component_name.to_owned())
}

pub fn generate_fn_name(path: &Path) -> String {
    let component_name = extract_component_name(path).unwrap_or_default();
    let path_bytes = path.as_os_str().as_encoded_bytes();

    let mut hash: u64 = 5381;
    for c in path_bytes {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(*c as u64);
    }

    format!("{}_{:x}", component_name, hash)
}

pub fn params_to_ts(params: &mut [(&str, &str)]) -> TokenStream {
    params.sort_by(|a, b| a.0.cmp(b.0));

    let args = params.iter().map(|(param_name, param_type)| {
        let param_name = Ident::new(param_name, Span::call_site());
        let param_type = TokenStream::from_str(param_type).unwrap();
        quote! { #param_name: #param_type }
    });
    quote! {#(#args),*}
}

pub fn param_names_to_ts<S>(param_names: &mut [S]) -> TokenStream
where
    S: AsRef<str> + Clone,
{
    param_names.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));

    let args = param_names
        .as_ref()
        .iter()
        .map(|param| Ident::new(param.as_ref(), Span::call_site()));
    quote! {#(#args),*}
}
