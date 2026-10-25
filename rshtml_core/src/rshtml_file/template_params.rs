use super::{Input, template::rust_identifier};
use crate::{context::Op, extensions::ParserDiagnostic};
use winnow::{
    ModalResult, Parser,
    ascii::{multispace0, multispace1},
    combinator::{alt, cut_err, not, opt, peek, repeat, separated},
    token::any,
};

pub fn template_params<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    ("@", multispace0, params, opt((multispace0, ';')))
        .void()
        .parse_next(input)
}

fn params<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    let span = (
        "(",
        multispace0,
        opt((peek(not(')')), separated(0.., param, ",")).map(|(_, params)| params)),
        multispace0,
        ")".expected(")"),
    )
        .map(|(_, _, params, _, _)| params.unwrap_or(Vec::new()))
        .span()
        .parse_next(input)?;

    input.state.ops.push(Op::TemplateParams(span));

    Ok(())
}

fn param<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    let (name_span, ty_span) = (
        multispace0,
        cut_err((opt(("self", multispace1)), rust_identifier).span()).expected("identifier"),
        cut_err(peek(alt((
            ':'.void(),
            ','.void(),
            ')'.void(),
            any.verify(|c: &char| c.is_whitespace()).void(),
        ))))
        .void()
        .expected("identifier"),
        multispace0,
        (
            ':',
            multispace0,
            cut_err(
                param_type
                    .with_span()
                    .verify(|(pt, _)| syn::parse_str::<syn::Type>(pt).is_ok()),
            )
            .expected("type"),
            multispace0,
        ),
    )
        .map(|(_, name_span, _, _, (_, _, (_, ty_span), _))| (name_span, ty_span))
        .parse_next(input)?;

    input
        .state
        .ops
        .push(Op::TemplateParam((name_span, ty_span)));

    Ok(())
}

fn param_type<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<&'a str> {
    repeat(
        1..,
        alt((
            param_type_nested,
            any.verify(|c: &char| !"([{<,)".contains(*c)).void(),
        )),
    )
    .fold(|| (), |_, _| ())
    .take()
    .parse_next(input)
}

fn param_type_nested<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    alt((
        (
            "(".void(),
            repeat(
                0..,
                alt((
                    param_type_nested.void(),
                    any.verify(|c: &char| *c != ')').void(),
                )),
            )
            .fold(|| (), |_, _| ()),
            ")".void(),
        ),
        (
            "[".void(),
            repeat(
                0..,
                alt((
                    param_type_nested.void(),
                    any.verify(|c: &char| *c != ']').void(),
                )),
            )
            .fold(|| (), |_, _| ()),
            "]".void(),
        ),
        (
            "{".void(),
            repeat(
                0..,
                alt((
                    param_type_nested.void(),
                    any.verify(|c: &char| *c != '}').void(),
                )),
            )
            .fold(|| (), |_, _| ()),
            "}".void(),
        ),
        (
            "<".void(),
            repeat(
                0..,
                alt((
                    param_type_nested.void(),
                    "->".void(),
                    "=>".void(),
                    any.verify(|c: &char| *c != '>').void(),
                )),
            )
            .fold(|| (), |_, _| ()),
            ">".void(),
        ),
    ))
    .void()
    .parse_next(input)
}
