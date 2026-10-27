use super::{
    Input, component::component, inner_text::inner_text, rust_block::rust_block,
    rust_stmt::rust_stmt, simple_expr::simple_expr, simple_expr_paren::simple_expr_paren,
    template_params::template_params, text::text, use_directive::use_directive,
};
use crate::{
    context::Op,
    extensions::ParserDiagnostic,
    rshtml_file::{
        break_directive::break_directive, child_content_directive::child_content_directive,
        continue_directive::continue_directive,
    },
};
use winnow::{
    ModalResult, Parser,
    ascii::multispace0,
    combinator::{alt, cut_err, eof, fail, opt, peek, repeat},
    token::{any, none_of, one_of, take_while},
};

pub fn template<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    (
        opt("\u{FEFF}"),
        multispace0,
        opt((peek(('@', multispace0, '(')), template_params.label("template parameters"))).void(),
        template_content,
        eof.expected("end of file"),
    )
        .map(|(_, _, _, content, _)| content)
        .parse_next(input)
}

pub fn template_content<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    repeat(0.., alt((text.label("html text"), block))).parse_next(input)
}

pub fn inner_template_content<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    let span = '{'.span().parse_next(input)?;
    input.state.ops.push(Op::OpenBrace(span));

    repeat(0.., alt((inner_text.label("html text"), block))).map(|()| ()).parse_next(input)?;

    let span = cut_err('}'.span()).expected("}").parse_next(input)?;
    input.state.ops.push(Op::CloseBrace(span));

    Ok(())
}

pub fn block<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    alt((
        component.label("component"),
        (
            '@',
            multispace0,
            alt((
                use_directive.label("use directive"),
                rust_block.label("code block"),
                rust_stmt.label("statement"),
                child_content_directive.label("child content"),
                continue_directive.label("continue"),
                break_directive.label("break"),
                simple_expr_paren.label("parenthesized expression"),
                simple_expr.label("expression"),
                cut_err(fail).expected("a valid directive or an expression after '@'"),
            )),
        )
            .void(),
        fail.expected("a valid directive or an expression or a component"),
    ))
    .parse_next(input)
}

pub fn rust_identifier<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<&'a str> {
    (
        one_of(|c: char| c.is_alphabetic() || c == '_'),
        take_while(0.., |c: char| c.is_alphanumeric() || c == '_'),
    )
        .take()
        .parse_next(input)
}

pub fn string_line<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<&'a str> {
    alt((
        (
            '"',
            repeat(0.., alt((("\\", any).void(), none_of(['"', '\\']).void())))
                .map(|_: Vec<()>| ()),
            '"',
        ),
        (
            '\'',
            repeat(0.., alt((("\\", any).void(), none_of(['\'', '\\']).void())))
                .map(|_: Vec<()>| ()),
            '\'',
        ),
    ))
    .void()
    .take()
    .parse_next(input)
}

pub fn component_tag_identifier<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<&'a str> {
    (
        any.verify(|c: &char| c.is_ascii_uppercase()),
        take_while(0.., |c: char| c.is_ascii_alphanumeric()),
    )
        .take()
        .parse_next(input)
}
