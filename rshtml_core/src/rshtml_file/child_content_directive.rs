use crate::{
    context::Op,
    rshtml_file::{Input, template::rust_identifier},
};
use winnow::{
    ModalResult, Parser,
    combinator::{alt, not, peek},
};

pub fn child_content_directive<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    let span = ("child_content", alt(("()".void(), peek(not(rust_identifier)).void()))).span().parse_next(input)?;

    input.state.ops.push(Op::ChildContent(span));

    Ok(())
}
