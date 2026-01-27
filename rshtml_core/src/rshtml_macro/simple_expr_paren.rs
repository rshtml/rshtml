use crate::rshtml_macro::{Input, template::escape_or_raw};
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

pub fn simple_expr_paren<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    let start = input.input;
    let checkpoint = input.checkpoint();

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
        .map(|(is_escaped_opt, _, _, _)| is_escaped_opt.is_some())
        .parse_next(input)?;

    let consumed = start.len() - input.len();
    let raw_expr = &start[..consumed];

    let start_offset = if is_escaped { 2 } else { 1 };
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

fn nested_expression<'a>(input: &mut Input<'a>) -> ModalResult<()> {
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
