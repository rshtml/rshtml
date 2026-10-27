use super::{Input, template::inner_template_content};
use crate::{context::Op, extensions::ParserDiagnostic};
use syn::parse_str;
use winnow::{
    ModalResult, Parser,
    ascii::{multispace0, multispace1},
    combinator::{alt, cut_err, opt, peek, repeat},
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    stream::Stream,
    token::take_till,
};

pub fn rust_stmt<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    alt((
        (
            |input: &mut Input<'a, 'ctx>| stmt("if", input),
            repeat(
                0..,
                (
                    multispace0,
                    peek(("else", multispace1, "if", multispace1)),
                    else_stmt,
                    multispace1,
                    |input: &mut Input<'a, 'ctx>| stmt("if", input),
                ),
            )
            .map(|()| ()),
            opt((
                multispace0,
                peek(("else", multispace0, "{")),
                else_stmt,
                multispace0,
                must_inner_template_content,
            )),
        )
            .void(),
        |input: &mut Input<'a, 'ctx>| stmt("for", input),
        |input: &mut Input<'a, 'ctx>| stmt("while", input),
    ))
    .parse_next(input)
}

fn else_stmt<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    let span = "else".span().parse_next(input)?;

    input.state.ops.push(Op::RustCode(span));

    Ok(())
}

fn stmt<'a, 'ctx>(keyword: &'static str, input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    let checkpoint = input.checkpoint();

    let (_, (head, span)) =
        (multispace0, (keyword, stmt_head).take().with_span()).parse_next(input)?;

    if let Err(e) = parse_str::<syn::Expr>(&format!("{} {{}}", head)) {
        let error_msg = Box::leak(e.to_string().into_boxed_str());
        input.reset(&checkpoint);
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &checkpoint,
            StrContext::Expected(StrContextValue::Description(error_msg)),
        )));
    }

    input.state.ops.push(Op::RustCode(span));

    must_inner_template_content.parse_next(input)
}

fn stmt_head<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    (multispace1, take_till(1.., ['{', '@', '}']), multispace0).void().parse_next(input)
}

fn must_inner_template_content<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    (cut_err(peek('{')).expected("{"), inner_template_content).void().parse_next(input)
}
