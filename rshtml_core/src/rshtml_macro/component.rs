use crate::rshtml_macro::{
    Input,
    extensions::ParserDiagnostic,
    simple_expr::simple_expr,
    simple_expr_paren::simple_expr_paren,
    template::{inner_template_content, string_line, template_content},
};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use std::str::FromStr;
use syn::{Ident, parse_str};
use winnow::{
    ModalResult, Parser,
    ascii::{alpha1, alphanumeric1, multispace0, multispace1},
    combinator::{alt, cut_err, opt, repeat},
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    prelude::*,
    token::{any, take_while},
};

// TODO: take component names order from use directive from file
pub fn component<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    let checkpoint = input.checkpoint();
    let mut ts = TokenStream::new();

    let (_, _, tag_name, (attributes, attribute_names), _, body) = (
        "<",
        multispace0,
        component_tag_identifier,
        attributes,
        multispace0,
        alt((
            "/>".map(|_| TokenStream::new()),
            (
                cut_err(">").expected(">"),
                template_content,
                cut_err("</").expected("</"),
                multispace0,
                cut_err(component_tag_identifier).expected("tag ident"),
                multispace0,
                cut_err(">").expected(">"),
            )
                .map(|(_, content, _, _, _, _, _)| content),
        )),
    )
        .parse_next(input)?;

    let use_directive = input
        .state
        .use_directives
        .iter()
        .find(|(name, _, _)| name == tag_name);

    let Some((_, _, fn_name)) = use_directive else {
        input.reset(&checkpoint);

        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &checkpoint,
            StrContext::Expected(StrContextValue::Description(
                "attempted to use an undeclared component",
            )),
        )));
    };

    let fn_name = Ident::new(fn_name, Span::call_site());

    ts.extend(attributes);
    ts.extend(quote! {let child_content = |__f__: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {#body  Ok(())};});

    let args = quote! {#(#attribute_names),*};
    ts.extend(quote! {self.#fn_name(__f__, child_content, #args)?;});

    Ok(quote! {{ #ts }})
}

fn attributes<'a>(input: &mut Input<'a>) -> ModalResult<(TokenStream, Vec<Ident>)> {
    repeat(0.., (multispace1, attribute))
        .fold(
            || (TokenStream::new(), Vec::new()),
            |mut acc, (_, (attr, name))| {
                acc.0.extend(attr);
                acc.1.push(name);
                acc
            },
        )
        .parse_next(input)
}

fn attribute<'a>(input: &mut Input<'a>) -> ModalResult<(TokenStream, Ident)> {
    let checkpoint = input.checkpoint();

    let (name, value) = (
        attribute_name,
        opt(((multispace0, "=", multispace0), attribute_value).map(|(_, value)| value)),
    )
        .parse_next(input)?;

    let name_ts = parse_str::<Ident>(name).map_err(|e| {
        input.reset(&checkpoint);
        let error_msg = Box::leak(e.to_string().into_boxed_str());
        ErrMode::Cut(ContextError::new().add_context(
            input,
            &checkpoint,
            StrContext::Expected(StrContextValue::Description(error_msg)),
        ))
    })?;

    let value = value.unwrap_or(true.to_token_stream());

    Ok((quote! {let #name_ts = #value;}, name_ts))
}

fn attribute_name<'a>(input: &mut Input<'a>) -> ModalResult<&'a str> {
    let start = input.input;

    (
        alt((alpha1.void(), "_".void())),
        repeat(0.., alt((alphanumeric1.void(), "_".void()))).fold(|| (), |_, _| ()),
    )
        .parse_next(input)?;

    let consumed = start.len() - input.input.len();
    Ok(&start[..consumed])
}

fn attribute_value<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    let checkpoint = input.checkpoint();

    let value_result = alt((
        alt(("true", "false")).map(|value| TokenStream::from_str(value)),
        winnow::ascii::float.map(|value: f64| Ok(value.to_token_stream())),
        string_line.map(|value| TokenStream::from_str(value)),
        ("@", alt((simple_expr_paren, simple_expr))).map(|(_, value)| Ok(value)),
        inner_template_content.map(|value| Ok(quote!{ ::rshtml::Expr(|__f__: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {#value Ok(())}) })),
    ))
    .parse_next(input)?;

    match value_result {
        Ok(value) => Ok(value),
        Err(e) => {
            input.reset(&checkpoint);
            let error_msg = Box::leak(e.to_string().into_boxed_str());
            Err(ErrMode::Cut(ContextError::new().add_context(
                input,
                &checkpoint,
                StrContext::Expected(StrContextValue::Description(error_msg)),
            )))
        }
    }
}

pub fn component_tag_identifier<'a>(input: &mut Input<'a>) -> ModalResult<&'a str> {
    let start = input.input;

    (
        any.verify(|c: &char| c.is_ascii_uppercase()),
        take_while(0.., |c: char| c.is_ascii_alphanumeric()),
    )
        .parse_next(input)?;

    let consumed = start.len() - input.len();

    Ok(&start[..consumed])
}
