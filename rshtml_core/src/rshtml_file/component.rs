use super::{
    Input,
    template::{component_tag_identifier, inner_template_content, string_line, template_content},
};
use crate::{
    context::Op,
    extensions::ParserDiagnostic,
    rshtml_file::{simple_expr::parse_simple_expr, simple_expr_paren::parse_simple_expr_paren},
};
use std::ops::Range;
use syn::{Ident, parse_str};
use winnow::{
    ModalResult, Parser,
    ascii::{alpha1, alphanumeric1, float, multispace0, multispace1},
    combinator::{alt, cut_err, opt, peek, repeat},
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    prelude::*,
};

pub fn component<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    (
        "<",
        multispace0,
        |input: &mut Input<'a, 'ctx>| tag_identifier(input, true),
        attributes,
        multispace0,
        alt((
            self_close,
            (
                close,
                template_content,
                cut_err("</").expected("</"),
                multispace0,
                cut_err(|input: &mut Input<'a, 'ctx>| tag_identifier(input, false)).expected("tag identifier"),
                multispace0,
                cut_err(">").expected(">"),
            )
                .void(),
        )),
    )
        .parse_next(input)?;

    Ok(())
}

fn close<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    cut_err(">").expected(">").parse_next(input)?;
    input.state.ops.push(Op::ComponentBodyStart);
    Ok(())
}

fn self_close<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    "/>".parse_next(input)?;
    input.state.ops.push(Op::ComponentEnd);
    Ok(())
}

fn attributes<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    repeat(0.., (multispace1, attribute)).parse_next(input)
}

fn attribute<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    let (name, _, _, _) = (attribute_name, multispace0, "=", multispace0).parse_next(input)?;

    if let Some(value) = opt(attribute_value).parse_next(input)? {
        input.state.ops.push(Op::ComponentAttribute(name, value));
        return Ok(());
    }

    input.state.ops.push(Op::ComponentAttribute(name, None));

    cut_err(peek('{')).expected("{").parse_next(input)?;
    inner_template_content.parse_next(input)?;

    input.state.ops.push(Op::ComponentAttributeEnd);

    Ok(())
}

fn attribute_name<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<Range<usize>> {
    let checkpoint = input.checkpoint();

    let (attribute_name, span) =
        (alt((alpha1.void(), "_".void())), repeat(0.., alt((alphanumeric1.void(), "_".void()))).fold(|| (), |_, _| ()))
            .take()
            .with_span()
            .parse_next(input)?;

    parse_str::<Ident>(attribute_name).map_err(|e| {
        input.reset(&checkpoint);
        let error_msg = Box::leak(e.to_string().into_boxed_str());
        ErrMode::Cut(ContextError::new().add_context(
            input,
            &checkpoint,
            StrContext::Expected(StrContextValue::Description(error_msg)),
        ))
    })?;

    Ok(span)
}

fn attribute_value<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<Option<Range<usize>>> {
    alt((
        alt(("true", "false")).span().map(Some),
        float::<_, f64, _>.span().map(Some),
        string_line.span().map(Some),
        ("@", alt((parse_simple_expr_paren, parse_simple_expr)).span()).map(|(_, expr)| Some(expr)),
    ))
    .parse_next(input)
}

pub fn tag_identifier<'a, 'ctx>(input: &mut Input<'a, 'ctx>, is_start: bool) -> ModalResult<()> {
    let span = component_tag_identifier.span().parse_next(input)?;

    if is_start {
        input.state.ops.push(Op::ComponentStartName(span));
    } else {
        input.state.ops.push(Op::ComponentEnd);
    }

    Ok(())
}
