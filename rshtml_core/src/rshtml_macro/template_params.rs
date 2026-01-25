use crate::rshtml_macro::{Context, rust_identifier};
use winnow::{
    ModalResult, Parser,
    combinator::{alt, cut_err, opt, repeat, separated},
    token::any,
};

pub fn template_params<'a>(
    input: &mut &'a str,
    ctx: &Context,
) -> ModalResult<Vec<(&'a str, Option<&'a str>)>> {
    ("@", (move |i: &mut &'a str| params(i, ctx)), opt("?"))
        .map(|(_, params, _)| params)
        .parse_next(input)
}

fn params<'a>(input: &mut &'a str, ctx: &Context) -> ModalResult<Vec<(&'a str, Option<&'a str>)>> {
    (
        "(",
        separated(0.., |i: &mut &'a str| param(i, ctx), ","),
        opt(","),
        ")",
    )
        .map(|(_, params, _, _)| params)
        .parse_next(input)
}

// TODO: add diagnostic message
fn param<'a>(input: &mut &'a str, ctx: &Context) -> ModalResult<(&'a str, Option<&'a str>)> {
    (
        cut_err(rust_identifier),
        opt((
            ":",
            cut_err(param_type.verify(|pt| syn::parse_str::<syn::Type>(pt).is_ok())),
        )),
    )
        .map(|(name, type_opt)| {
            let type_str = type_opt.map(|(_, ty)| ty);
            (name, type_str)
        })
        .parse_next(input)
}

fn param_type<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    let start = *input;

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

fn param_type_nested(input: &mut &str) -> ModalResult<()> {
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
