use crate::{context::Op, rshtml_file::Input};
use winnow::{ModalResult, Parser, ascii::multispace0};

pub fn break_directive<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    let span = ("break", multispace0).span().parse_next(input)?;

    input.state.ops.push(Op::ContinueOrBreak(span));

    Ok(())
}
