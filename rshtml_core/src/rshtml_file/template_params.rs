use super::{Input, template::rust_identifier};
use crate::extensions::ParserDiagnostic;
use winnow::{
    ModalResult, Parser,
    ascii::multispace0,
    combinator::{alt, cut_err, not, opt, peek, repeat, separated},
    token::any,
};
// TODO: validate param name as ident and param type as type syn parse
pub fn template_params<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    let parsed_params: Vec<(String, String)> = ("@", multispace0, params, opt((multispace0, ';')))
        .map(|(_, _, params, _)| {
            params
                .iter()
                .map(|(p_name, p_type_opt)| {
                    (
                        p_name.to_string(),
                        p_type_opt.unwrap_or("impl ::rshtml::View").to_string(),
                    )
                })
                .collect()
        })
        .parse_next(input)?;

    input.state.info.template_params = parsed_params;

    Ok(())
}

fn params<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<Vec<(&'a str, Option<&'a str>)>> {
    (
        "(",
        multispace0,
        opt((peek(not(')')), separated(0.., param, ",")).map(|(_, params)| params)),
        multispace0,
        ")".expected(")"),
    )
        .map(|(_, _, params, _, _)| params.unwrap_or(Vec::new()))
        .parse_next(input)
}

fn param<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<(&'a str, Option<&'a str>)> {
    (
        multispace0,
        cut_err(rust_identifier).expected("identifier"),
        cut_err(peek(alt((
            ':'.void(),
            ','.void(),
            ')'.void(),
            any.verify(|c: &char| c.is_whitespace()).void(),
        ))))
        .void()
        .expected("identifier"),
        multispace0,
        opt((
            ':',
            multispace0,
            cut_err(param_type.verify(|pt| syn::parse_str::<syn::Type>(pt).is_ok()))
                .expected("type"),
            multispace0,
        )),
    )
        .map(|(_, name, _, _, type_opt)| {
            let type_str = type_opt.map(|(_, _, ty, _)| ty);
            (name, type_str)
        })
        .parse_next(input)
}

fn param_type<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<&'a str> {
    let start = input.input;

    repeat(
        1..,
        alt((
            param_type_nested,
            any.verify(|c: &char| !"([{<,)".contains(*c)).void(),
        )),
    )
    .fold(|| (), |_, _| ())
    .parse_next(input)?;

    let consumed = start.len() - input.len();
    Ok(&start[..consumed])
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
