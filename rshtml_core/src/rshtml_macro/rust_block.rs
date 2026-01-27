use crate::rshtml_macro::Input;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;
use winnow::{
    ModalResult, Parser,
    ascii::multispace0,
    combinator::{alt, cut_err, not, repeat},
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    stream::Stream,
    token::{any, none_of, take_while},
};

pub fn rust_block<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    let start = input.input;
    let checkpoint = input.checkpoint();

    ('@'.void(), multispace0.void(), block).parse_next(input)?;

    let len = start.len() - input.len();
    let rust_block = &start[1..len];

    let output = match parse_str::<syn::Block>(rust_block) {
        Ok(block) => {
            let stmts = &block.stmts;
            quote! { #(#stmts)* }
        }
        Err(e) => {
            let span = e.span();
            let start = span.start();

            let offset = rust_block
                .split_inclusive('\n')
                .take(start.line - 1)
                .map(|line| line.len())
                .sum::<usize>()
                + start.column;
            input.reset(&checkpoint);

            let _ = input.next_slice(offset);

            let error_msg = Box::leak(e.to_string().into_boxed_str());

            return Err(ErrMode::Cut(ContextError::new().add_context(
                input,
                &checkpoint,
                StrContext::Expected(StrContextValue::Description(error_msg)),
            )));
        }
    };

    Ok(output)
}

fn block<'a>(input: &mut Input<'a>) -> ModalResult<()> {
    (
        "{",
        block_content,
        cut_err("}").context(StrContext::Expected(StrContextValue::CharLiteral('}'))),
    )
        .void()
        .parse_next(input)
}

fn block_content<'a>(input: &mut Input<'a>) -> ModalResult<()> {
    repeat(
        0..,
        alt((
            block.void(),
            line_comment,
            block_comment,
            string_literal,
            char_literal,
            take_while(1.., |c| !r#"{}"'/"#.contains(c)).void(),
            none_of('}').void(),
        )),
    )
    .parse_next(input)
}

fn line_comment<'a>(input: &mut Input<'a>) -> ModalResult<()> {
    ("//", take_while(0.., |c| c != '\n'))
        .void()
        .parse_next(input)
}

fn block_comment<'a>(input: &mut Input<'a>) -> ModalResult<()> {
    (
        "/*",
        repeat(
            0..,
            alt((
                block_comment,
                take_while(1.., |c| c != '/' && c != '*').void(),
                (not(alt(("/*", "*/"))), any).void(),
            )),
        )
        .fold(|| (), |_, _| ()),
        "*/",
    )
        .void()
        .parse_next(input)
}

fn string_literal<'a>(input: &mut Input<'a>) -> ModalResult<()> {
    (
        '"',
        repeat(0.., alt((("\\", any).void(), none_of(['"', '\\']).void()))).map(|_: Vec<()>| ()),
        '"',
    )
        .void()
        .parse_next(input)
}

fn char_literal<'a>(input: &mut Input<'a>) -> ModalResult<()> {
    (
        '\'',
        alt((("\\", any).void(), none_of(['"', '\\']).void())),
        '\'',
    )
        .void()
        .parse_next(input)
}
