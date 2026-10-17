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

#[cfg(not(debug_assertions))]
pub fn rust_stmt<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<TokenStream> {
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
        while_stmt,
    ))
    .parse_next(input)
}

#[cfg(debug_assertions)]
pub fn rust_stmt<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<TokenStream> {
    alt((
        (
            if_stmt,
            repeat(0.., (multispace0, else_stmt, multispace1, if_stmt)).fold(
                TokenStream::new,
                |mut acc, (_, _, _, if_stmt_)| {
                    acc.extend(quote! { else });
                    acc.extend(if_stmt_);
                    acc
                },
            ),
            opt((
                multispace0,
                else_stmt,
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
        while_stmt,
    ))
    .parse_next(input)
}

#[cfg(debug_assertions)]
fn else_stmt<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<()> {
    use crate::debug::Snippet;

    let (text, span) = "else".with_span().parse_next(input)?;

    input
        .state
        .info
        .debug
        .push(Snippet::Stmt(span, text.to_string()));

    Ok(())
}

#[cfg(not(debug_assertions))]
fn if_stmt<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<TokenStream> {
    let checkpoint = input.checkpoint();

    let (head, body) = (
        multispace0,
        ("if", stmt_head).with_taken(),
        must_inner_template_content,
    )
        .map(|(_, ((_, _), if_head), content)| (if_head, content))
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

#[cfg(debug_assertions)]
fn if_stmt<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<TokenStream> {
    use crate::debug::Snippet;

    let checkpoint = input.checkpoint();

    let (head, head_span, body) = (
        multispace0,
        ("if", stmt_head).with_taken().with_span(),
        must_inner_template_content,
    )
        .map(|(_, (((_, _), if_head), span), content)| (if_head, span, content))
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

    input
        .state
        .info
        .debug
        .push(Snippet::Stmt(head_span, head.to_string()));

    Ok(ts)
}

#[cfg(not(debug_assertions))]
fn for_stmt<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<TokenStream> {
    let checkpoint = input.checkpoint();

    let (head, body) = (
        multispace0,
        ("for", stmt_head).with_taken(),
        must_inner_template_content,
    )
        .map(|(_, ((_, _), for_head), content)| (for_head, content))
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

#[cfg(debug_assertions)]
fn for_stmt<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<TokenStream> {
    use crate::debug::Snippet;

    let checkpoint = input.checkpoint();

    let (head, head_span, body) = (
        multispace0,
        ("for", stmt_head).with_taken().with_span(),
        must_inner_template_content,
    )
        .map(|(_, (((_, _), for_head), span), content)| (for_head, span, content))
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

    input
        .state
        .info
        .debug
        .push(Snippet::Stmt(head_span, head.to_string()));

    Ok(ts)
}

#[cfg(not(debug_assertions))]
fn while_stmt<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<TokenStream> {
    let checkpoint = input.checkpoint();

    let (head, body) = (
        multispace0,
        ("while", stmt_head).with_taken(),
        must_inner_template_content,
    )
        .map(|(_, ((_, _), while_head), content)| (while_head, content))
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

#[cfg(debug_assertions)]
fn while_stmt<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<TokenStream> {
    use crate::debug::Snippet;

    let checkpoint = input.checkpoint();

    let (head, head_span, body) = (
        multispace0,
        ("while", stmt_head).with_taken().with_span(),
        must_inner_template_content,
    )
        .map(|(_, (((_, _), while_head), span), content)| (while_head, span, content))
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

    input
        .state
        .info
        .debug
        .push(Snippet::Stmt(head_span, head.to_string()));

    Ok(ts)
}

fn stmt_head<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<&'a str> {
    (
        multispace1,
        repeat(1.., none_of(['{', '@', '}'])).fold(|| (), |_, _| ()),
        multispace0,
    )
        .take()
        .parse_next(input)
}

fn must_inner_template_content<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<TokenStream> {
    (cut_err(peek('{')).expected("{"), inner_template_content)
        .map(|(_, content)| content)
        .parse_next(input)
}
