use std::ops::Range;

use super::Input;
use crate::context::Op;
use syn::{Expr, parse_str};
use winnow::{
    ModalResult, Parser,
    combinator::{alt, opt, repeat},
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    stream::Stream,
    token::any,
};

pub fn simple_expr_paren<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    let (span, is_escaped) = parse_simple_expr_paren.parse_next(input)?;

    input.state.ops.push(Op::ExprParen(span, is_escaped));

    Ok(())
}
pub fn parse_simple_expr_paren<'a, 'ctx>(
    input: &mut Input<'a, 'ctx>,
) -> ModalResult<(Range<usize>, bool)> {
    let checkpoint = input.checkpoint();

    let (is_not_escaped_opt, _, (expr, span), _) = (
        opt('#'),
        '(',
        repeat(
            0..,
            alt((nested_expression, any.verify(|c| *c != ')').void())),
        )
        .map(|()| ())
        .take()
        .with_span(),
        ')',
    )
        .parse_next(input)?;

    let is_escaped = is_not_escaped_opt.is_none();

    parse_str::<Expr>(expr).map_err(|_| {
        input.reset(&checkpoint);

        ErrMode::Cut(ContextError::new().add_context(
            input,
            &checkpoint,
            StrContext::Expected(StrContextValue::Description("invalid expression")),
        ))
    })?;

    Ok((span, is_escaped))
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
