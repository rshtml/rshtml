use crate::rshtml_macro::Context;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::parse_str;
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::not;
use winnow::combinator::repeat;
use winnow::error::AddContext;
use winnow::error::ContextError;
use winnow::error::ErrMode;
use winnow::error::StrContext;
use winnow::error::StrContextValue;
use winnow::token::any;
use winnow::token::none_of;
use winnow::token::take_while;
use winnow::{ModalResult, Parser, ascii::multispace0, stream::Stream};

pub fn rust_block(input: &mut &str, ctx: &Context) -> ModalResult<(TokenStream, TokenStream)> {
    let start = *input;
    let checkpoint = input.checkpoint();

    ("@", multispace0, block).parse_next(input)?;

    let len = start.len() - input.len();
    let rust_block = &start[1..len];

    let def_ident = format_ident!("_exp{}", input.len());

    let output = match parse_str::<syn::Block>(rust_block) {
        Ok(block) => (
            quote! { let #def_ident = #block; _text_size += ::rshtml::TextSize(&#def_ident).text_size(); },
            quote! { ::rshtml::Exp(&(#def_ident)).render(out)?; },
        ),
        Err(e) => {
            let span = e.span();
            let start = span.start();
            let end = span.end();

            // ctx.diagnostic.caution(, position, title, lines, info, name_len)

            let tokens: TokenStream = rust_block.parse().map_err(|_| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &checkpoint,
                    StrContext::Label("Lex Error"),
                ))
            })?;

            (
                quote! { let #def_ident = #tokens; _text_size += ::rshtml::TextSize(&#def_ident).text_size(); },
                quote! { ::rshtml::Exp(&(#def_ident)).render(out)?; },
            )
        }
    };

    Ok(output)
}

fn block(input: &mut &str) -> ModalResult<()> {
    (
        "{",
        block_content,
        cut_err("}").context(StrContext::Expected(StrContextValue::CharLiteral('}'))),
    )
        .void()
        .parse_next(input)
}

fn block_content(input: &mut &str) -> ModalResult<()> {
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

fn line_comment(input: &mut &str) -> ModalResult<()> {
    ("//", take_while(0.., |c| c != '\n'))
        .void()
        .parse_next(input)
}

fn block_comment(input: &mut &str) -> ModalResult<()> {
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

fn string_literal(input: &mut &str) -> ModalResult<()> {
    (
        '"',
        repeat(0.., alt((("\\", any).void(), none_of(['"', '\\']).void()))).map(|_: Vec<()>| ()),
        '"',
    )
        .void()
        .parse_next(input)
}

fn char_literal(input: &mut &str) -> ModalResult<()> {
    (
        '\'',
        alt((("\\", any).void(), none_of(['"', '\\']).void())),
        '\'',
    )
        .void()
        .parse_next(input)
}
