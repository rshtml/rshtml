use super::{
    Input,
    template::{rust_identifier, string_line},
    utils::{escape_or_raw, get_struct_field},
};
use crate::{diagnostic::Diagnostic, position::Position};
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

pub fn simple_expr<'a, 'ctx>(
    input: &mut Input<'a, 'ctx>,
) -> ModalResult<(TokenStream, TokenStream)> {
    let checkpoint = input.checkpoint();

    let len = input.state.source.len().saturating_sub(input.input.len());
    let position: Position = (
        input.state.source,
        input.state.source.len().saturating_sub(input.input.len()),
    )
        .into();

    let message = Diagnostic(input.state.source).message(
        input.state.path,
        &position,
        &format!(
            "attempt to use an expression that does not implement the Display trait. {position:?} {len}"
        ),
        "this expression does not implement the Display trait.",
        1,
    );

    not((
        multispace0,
        alt(("{", "(", "if", "for", "use", "child_content")),
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

    if let Some(field) = get_struct_field(expr)
        && !input.state.struct_fields.contains(&field)
    {
        input.reset(&checkpoint);

        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &checkpoint,
            StrContext::Expected(StrContextValue::Description(
                "attempt to use undefined struct field",
            )),
        )));
    }

    let expr_ts = escape_or_raw(quote!(#expression), is_escaped, &message);

    Ok((quote! {#expression}, expr_ts))
}

fn chain_segment<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
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

fn nested_content<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    alt((
        ("(", repeat(0.., nested_content).fold(|| (), |_, _| ()), ")").void(),
        ("[", repeat(0.., nested_content).fold(|| (), |_, _| ()), "]").void(),
        string_line.void(),
        (not(alt((")".void(), "]".void(), expression_boundary))), any).void(),
    ))
    .parse_next(input)
}

fn expression_boundary<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    alt((
        ("<", alt(("/", winnow::ascii::alpha1))).void(),
        "@".void(),
        "{".void(),
        winnow::ascii::line_ending.void(),
    ))
    .void()
    .parse_next(input)
}
