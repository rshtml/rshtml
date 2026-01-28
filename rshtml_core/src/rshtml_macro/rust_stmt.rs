use crate::rshtml_macro::{Input, template::inner_template_content};
use proc_macro2::TokenStream;
use syn::parse_str;
use winnow::{
    ModalResult, Parser,
    ascii::{multispace0, multispace1},
    combinator::{alt, cut_err, repeat},
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    stream::Stream,
    token::{any, none_of},
};

pub fn rust_stmt<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    let items = repeat(1.., |i: &mut Input<'a>| {
        let cp = i.checkpoint();
        let (_, head, _, body) = (
            multispace0,
            rust_stmt_head,
            multispace0,
            inner_template_content,
        )
            .parse_next(i)?;
        Ok((cp, head, body))
    })
    .fold(Vec::new, |mut acc, item| {
        acc.push(item);
        acc
    })
    .parse_next(input)?;

    let mut acc_ts = TokenStream::new();
    let mut acc_str = String::new();

    for (checkpoint, head, body) in items {
        acc_str.push_str(&format!("{} {{}}", head));

        if let Err(e) = parse_str::<syn::Expr>(&acc_str) {
            let error_msg = Box::leak(e.to_string().into_boxed_str());
            input.reset(&checkpoint);
            return Err(ErrMode::Cut(ContextError::new().add_context(
                input,
                &checkpoint,
                StrContext::Expected(StrContextValue::Description(error_msg)),
            )));
        };

        let head_ts: TokenStream = head.parse().unwrap();

        acc_ts.extend(head_ts);
        acc_ts.extend(body);
    }

    Ok(acc_ts)
}

fn rust_stmt_head<'a>(input: &mut Input<'a>) -> ModalResult<&'a str> {
    let start = input.input;

    alt((
        (
            alt((
                "if".void(),
                ("else", multispace1, "if").void(),
                "for".void(),
            )),
            multispace1,
            repeat(1.., none_of(['{', '@', '}'])).fold(|| (), |_, _| ()),
        )
            .void(),
        "else".void(),
    ))
    .parse_next(input)?;

    let len = start.len() - input.input.len();
    let head_str = &start[..len];

    Ok(head_str)
}
