use crate::rshtml_macro::{
    Input,
    template::{escape_or_raw, rust_identifier, string_line},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, parse_str};
use winnow::{
    ModalResult, Parser,
    ascii::{multispace0, multispace1},
    combinator::{alt, not, opt, repeat},
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    stream::Stream,
    token::any,
};

pub fn simple_expr<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    let checkpoint = input.checkpoint();

    not((
        multispace0,
        alt(("{", "if", "for", "use", "child_content")),
        alt((multispace1.void(), not(rust_identifier).void())),
    ))
    .parse_next(input)?;

    let start = input.input;

    let is_escaped = (
        opt("#"),
        repeat(0.., "&").fold(|| (), |_, _| ()),
        rust_identifier,
        repeat(0.., chain_segment).fold(|| (), |_, _| ()),
    )
        .map(|(is_not_escaped, _, _, _)| is_not_escaped.is_none())
        .parse_next(input)?;

    let len = start.len() - input.input.len();
    let raw_expr = &start[..len];

    let start_offset = if is_escaped { 0 } else { 1 };
    let expr = &raw_expr[start_offset..];

    let expression = parse_str::<Expr>(expr).map_err(|_| {
        input.reset(&checkpoint);

        ErrMode::Cut(ContextError::new().add_context(
            input,
            &checkpoint,
            StrContext::Expected(StrContextValue::Description("invalid expression")),
        ))
    })?;

    // let message = input.state.diagnostic.caution(
    //     &input.state.diagnostic.sources,
    //     &position,
    //     "attempt to use an expression that does not implement the Display trait.",
    //     &[],
    //     "this expression does not implement the Display trait.",
    //     expr.len(),
    // );

    let expr_ts = escape_or_raw(quote!(#expression), is_escaped, ""); // TODO: Add Diagnostic Message

    Ok(expr_ts)
}

fn chain_segment<'a>(input: &mut Input<'a>) -> ModalResult<()> {
    alt((
        ("&", rust_identifier).void(),
        (".", rust_identifier).void(),
        ("::", rust_identifier).void(),
        ("(", repeat(0.., nested_content).fold(|| (), |_, _| ()), ")").void(),
        ("[", repeat(0.., nested_content).fold(|| (), |_, _| ()), "]").void(),
    ))
    .void()
    .parse_next(input)
}

fn nested_content<'a>(input: &mut Input<'a>) -> ModalResult<()> {
    alt((
        ("(", repeat(0.., nested_content).fold(|| (), |_, _| ()), ")").void(),
        ("[", repeat(0.., nested_content).fold(|| (), |_, _| ()), "]").void(),
        string_line.void(),
        (not(alt((")".void(), "]".void(), expression_boundary))), any).void(),
    ))
    .parse_next(input)
}

fn expression_boundary<'a>(input: &mut Input<'a>) -> ModalResult<()> {
    alt((
        ("<", alt(("/", winnow::ascii::alpha1))).void(),
        "@".void(),
        "{".void(),
        winnow::ascii::line_ending.void(),
    ))
    .void()
    .parse_next(input)
}
