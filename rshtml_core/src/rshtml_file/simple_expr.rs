use super::{
    Input,
    template::{rust_identifier, string_line},
};
use crate::context::Op;
use std::ops::Range;
use winnow::{
    ModalResult, Parser,
    ascii::{multispace0, multispace1},
    combinator::{alt, not, opt, repeat},
    token::any,
};

pub fn simple_expr<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    let (span, is_escaped) = parse_simple_expr.parse_next(input)?;

    input.state.ops.push(Op::ExprSimple(span, is_escaped));

    Ok(())
}

pub fn parse_simple_expr<'a, 'ctx>(
    input: &mut Input<'a, 'ctx>,
) -> ModalResult<(Range<usize>, bool)> {
    not((
        multispace0,
        alt(("{", "(", "if", "for", "use", "child_content")),
        alt((multispace1.void(), not(rust_identifier).void())),
    ))
    .parse_next(input)?;

    let (is_not_escaped_opt, span) = (
        opt("#"),
        (
            repeat(0.., "&").fold(|| (), |_, _| ()),
            rust_identifier,
            repeat(0.., chain_segment).fold(|| (), |_, _| ()),
        )
            .span(),
    )
        .parse_next(input)?;

    let is_escaped = is_not_escaped_opt.is_none();

    Ok((span, is_escaped))
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
