use super::{Input, template::inner_template_content};
use crate::extensions::ParserDiagnostic;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;
use winnow::{
    ModalResult, Parser,
    ascii::{multispace0, multispace1},
    combinator::{alt, cut_err, opt, peek, repeat},
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    stream::Stream,
    token::none_of,
};

pub fn rust_stmt<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    alt((
        (
            if_stmt,
            repeat(0.., (multispace0, "else", multispace1, if_stmt)).fold(
                TokenStream::new,
                |mut acc, (_, _, _, if_stmt_)| {
                    acc.extend(quote! { else });
                    acc.extend(if_stmt_);
                    acc
                },
            ),
            opt((
                multispace0,
                "else",
                multispace0,
                must_inner_template_content,
            )
                .map(|(_, _, _, content)| {
                    quote! {else #content}
                })),
        )
            .map(|(if_, if_else_chain_, else_opt)| {
                let mut ts = TokenStream::new();
                ts.extend(if_);
                ts.extend(if_else_chain_);
                ts.extend(else_opt);

                ts
            }),
        for_stmt,
    ))
    .parse_next(input)
}

fn if_stmt<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    let checkpoint = input.checkpoint();

    let (head, body) = (multispace0, "if", stmt_head, must_inner_template_content)
        .map(|(_, if_, head, content)| (format!("{if_} {head}"), content))
        .parse_next(input)?;

    if let Err(e) = parse_str::<syn::Expr>(&format!("{} {{}}", head)) {
        let error_msg = Box::leak(e.to_string().into_boxed_str());
        input.reset(&checkpoint);
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &checkpoint,
            StrContext::Expected(StrContextValue::Description(error_msg)),
        )));
    };

    let head_ts: TokenStream = head.parse().unwrap();

    let mut ts = TokenStream::new();
    ts.extend(head_ts);
    ts.extend(body);

    Ok(ts)
}

fn for_stmt<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    let checkpoint = input.checkpoint();

    let (head, body) = (multispace0, "for", stmt_head, must_inner_template_content)
        .map(|(_, for_, head, content)| (format!("{for_} {head}"), content))
        .parse_next(input)?;

    if let Err(e) = parse_str::<syn::Expr>(&format!("{} {{}}", head)) {
        let error_msg = Box::leak(e.to_string().into_boxed_str());
        input.reset(&checkpoint);
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &checkpoint,
            StrContext::Expected(StrContextValue::Description(error_msg)),
        )));
    };

    let head_ts: TokenStream = head.parse().unwrap();

    let mut ts = TokenStream::new();
    ts.extend(head_ts);
    ts.extend(body);

    Ok(ts)
}

fn stmt_head<'a>(input: &mut Input<'a>) -> ModalResult<&'a str> {
    let start = input.input;

    (
        multispace1,
        repeat(1.., none_of(['{', '@', '}'])).fold(|| (), |_, _| ()),
        multispace0,
    )
        .void()
        .parse_next(input)?;

    let len = start.len() - input.input.len();
    let head_str = &start[..len];

    Ok(head_str)
}

fn must_inner_template_content<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    (cut_err(peek('{')).expected("{"), inner_template_content)
        .map(|(_, content)| content)
        .parse_next(input)
}
