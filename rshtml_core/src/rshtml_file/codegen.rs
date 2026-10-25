use crate::{diagnostic::Diagnostic, position::Position, rshtml_file::utils::escape_or_raw};
use proc_macro2::TokenStream;
use quote::quote;
use std::{ops::Range, path::Path};
use syn::{Expr, parse_str};

pub fn template_param<'a>(
    source: &'a str,
    range: (Range<usize>, Range<usize>),
    params: &mut Vec<(&'a str, &'a str)>,
) {
    let name = &source[range.0];
    let ty = &source[range.1];

    params.push((name, ty));
}

pub fn rust_code(source: &str, range: Range<usize>) -> Result<TokenStream, String> {
    let code = &source[range.start..range.end];
    code.parse::<TokenStream>().map_err(|e| e.to_string())
}

pub fn text(source: &str, range: Range<usize>) -> (TokenStream, usize) {
    let text = &source[range.start..range.end];
    let text_size = text.len();

    (quote! { __out__.write_str(#text)?; }, text_size)
}

pub fn child_content() -> TokenStream {
    quote! {child_content(__out__)?;}
}

pub fn continue_or_break(source: &str, range: Range<usize>) -> TokenStream {
    let continue_or_break = &source[range.start..range.end];
    if continue_or_break == "continue" {
        quote! { continue }
    } else {
        quote! { break }
    }
}

pub fn expr_simple_or_paren(
    source: &str,
    range: Range<usize>,
    is_escaped: bool,
    path: &Path,
) -> Result<TokenStream, String> {
    let position: Position = (source, &range).into();

    let expr = &source[range.start..range.end];

    let message = Diagnostic(source).message(
        path,
        &position,
        "attempt to use an expression that does not implement the Display trait.",
        "this expression does not implement the Display trait.",
        1,
    );

    let expression = parse_str::<Expr>(expr).map_err(|e| {
        Diagnostic(source).message(
            path,
            &position,
            "invalid expression",
            &e.to_string(),
            expr.len(),
        )
    })?;

    let expr_ts = escape_or_raw(quote!(#expression), is_escaped, &message);

    Ok(expr_ts)
}
