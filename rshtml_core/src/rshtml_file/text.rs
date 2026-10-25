use super::Input;
use crate::{context::Op, rshtml_file::template::component_tag_identifier};
use winnow::{
    ModalResult, Parser,
    ascii::multispace0,
    combinator::{alt, not, repeat},
    token::{any, take_while},
};

pub fn text<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    let span = repeat(
        1..,
        alt((
            take_while(1.., |c| c != '@' && c != '<').void(),
            "@@".void(),
            (
                not(alt((
                    '@'.void(),
                    ("<", multispace0, component_tag_identifier).void(),
                    ("</", multispace0, component_tag_identifier).void(),
                ))),
                any,
            )
                .void(),
        )),
    )
    .map(|()| ())
    .span()
    .parse_next(input)?;

    input.state.ops.push(Op::Text(span));

    Ok(())
}
