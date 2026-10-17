use super::{Input, template::rust_identifier};
use crate::extensions::ParserDiagnostic;
use winnow::{
    ModalResult, Parser,
    ascii::{multispace0, multispace1},
    combinator::{alt, cut_err, not, opt, peek, repeat, separated},
    token::any,
};

#[cfg(not(debug_assertions))]
pub fn template_params<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    let parsed_params: Vec<(String, String)> = ("@", multispace0, params, opt((multispace0, ';')))
        .map(|(_, _, params, _)| {
            params
                .into_iter()
                .map(|(name, ty)| (name.to_string(), ty.to_string()))
                .collect()
        })
        .parse_next(input)?;

    input.state.info.template_params = parsed_params;

    Ok(())
}

#[cfg(debug_assertions)]
pub fn template_params<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    use crate::debug::Snippet;

    let (parsed_params, params_str, span) = (
        "@",
        multispace0,
        params.with_taken().with_span(),
        opt((multispace0, ';')),
    )
        .map(|(_, _, ((params, params_str), span), _)| (params, params_str, span))
        .parse_next(input)?;

    let mut params_text = params_str.to_string();
    let mut params = Vec::with_capacity(parsed_params.len());

    for (name, ty) in parsed_params {
        if name.starts_with("self") && name[4..].starts_with(|c: char| c.is_whitespace()) {
            let offset = (name.as_ptr() as usize) - (params_str.as_ptr() as usize);
            params_text.replace_range(offset..offset + 4, "    ");
        }

        params.push((name.to_string(), ty.to_string()));
    }

    input.state.info.template_params = params;
    input
        .state
        .info
        .debug
        .push(Snippet::Params(span, params_text));

    Ok(())
}

fn params<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<Vec<(&'a str, &'a str)>> {
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

fn param<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<(&'a str, &'a str)> {
    (
        multispace0,
        cut_err((opt(("self", multispace1)), rust_identifier).take()).expected("identifier"),
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
            cut_err(param_type.verify(|pt| syn::parse_str::<syn::Type>(pt).is_ok()))
                .expected("type"),
            multispace0,
        ),
    )
        .map(|(_, name, _, _, (_, _, ty, _))| (name, ty))
        .parse_next(input)
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
