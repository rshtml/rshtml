use crate::rshtml_macro::{Input, extensions::ParserDiagnostic, template::rust_identifier};
use winnow::{
    ModalResult, Parser,
    combinator::{alt, cut_err, opt, peek, repeat, separated},
    token::any,
};

pub fn template_params<'a>(input: &mut Input<'a>) -> ModalResult<()> {
    let parsed_params: Vec<(String, String)> = ("@", params, opt("?"))
        .map(|(_, params, _)| {
            params
                .iter()
                .map(|(p_name, p_type_opt)| {
                    (
                        p_name.to_string(),
                        p_type_opt.unwrap_or("impl ::std::fmt::Display").to_string(),
                    )
                })
                .collect()
        })
        .parse_next(input)?;

    input.state.template_params = parsed_params;

    Ok(())
}

fn params<'a>(input: &mut Input<'a>) -> ModalResult<Vec<(&'a str, Option<&'a str>)>> {
    ("(", separated(0.., param, ","), opt(","), ")".expected(")"))
        .map(|(_, params, _, _)| params)
        .parse_next(input)
}

fn param<'a>(input: &mut Input<'a>) -> ModalResult<(&'a str, Option<&'a str>)> {
    (
        cut_err(rust_identifier).expected("identifier"),
        cut_err(peek(alt((
            ':'.void(),
            ','.void(),
            ')'.void(),
            any.verify(|c: &char| c.is_whitespace()).void(),
        ))))
        .void()
        .expected("identifier"),
        opt((
            ":",
            cut_err(param_type.verify(|pt| syn::parse_str::<syn::Type>(pt).is_ok()))
                .expected("type"),
        )),
    )
        .map(|(name, _, type_opt)| {
            let type_str = type_opt.map(|(_, ty)| ty);
            (name, type_str)
        })
        .parse_next(input)
}

fn param_type<'a>(input: &mut Input<'a>) -> ModalResult<&'a str> {
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

fn param_type_nested<'a>(input: &mut Input<'a>) -> ModalResult<()> {
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
