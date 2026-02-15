use super::{
    Input,
    utils::{escape_or_raw, get_struct_field},
};
use crate::{diagnostic::Diagnostic, position::Position};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, parse_str};
use winnow::{
    ModalResult, Parser,
    combinator::{alt, opt, repeat},
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    stream::Stream,
    token::any,
};

pub fn simple_expr_paren<'a, 'ctx>(
    input: &mut Input<'a, 'ctx>,
) -> ModalResult<(TokenStream, TokenStream)> {
    let start = input.input;
    let checkpoint = input.checkpoint();

    let position: Position = (
        input.state.source,
        input.state.source.len().saturating_sub(input.input.len()),
    )
        .into();

    let message = Diagnostic(input.state.source).message(
        input.state.path,
        &position,
        "attempt to use an expression that does not implement the Display trait.",
        "this expression does not implement the Display trait.",
        1,
    );

    let is_escaped = (
        opt('#'),
        '(',
        repeat(
            0..,
            alt((nested_expression, any.verify(|c| *c != ')').void())),
        )
        .fold(|| (), |_, _| ()),
        ')',
    )
        .map(|(is_not_escaped_opt, _, _, _)| is_not_escaped_opt.is_none())
        .parse_next(input)?;

    let consumed = start.len() - input.len();
    let raw_expr = &start[..consumed];

    let start_offset = if is_escaped { 1 } else { 2 };
    let end_offset = raw_expr.len() - 1;
    let expr = &raw_expr[start_offset..end_offset];

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

fn nested_expression<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    alt((
        (
            '(',
            repeat(
                0..,
                alt((nested_expression, any.verify(|c| *c != ')').void())),
            )
            .fold(|| (), |_, _| ()),
            ')',
        ),
        (
            '[',
            repeat(
                0..,
                alt((nested_expression, any.verify(|c| *c != ']').void())),
            )
            .fold(|| (), |_, _| ()),
            ']',
        ),
        (
            '{',
            repeat(
                0..,
                alt((nested_expression, any.verify(|c| *c != '}').void())),
            )
            .fold(|| (), |_, _| ()),
            '}',
        ),
    ))
    .void()
    .parse_next(input)
}
