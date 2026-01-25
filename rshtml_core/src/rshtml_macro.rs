mod inner_text;
mod rust_block;
mod template;
mod template_params;
mod text;

use crate::diagnostic::Diagnostic;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use std::{collections::HashMap, env, fs, path::PathBuf};
use syn::{LitStr, spanned::Spanned};
use template::template;
use winnow::{
    ModalResult, Parser,
    combinator::eof,
    error::StrContext,
    stream::Stream,
    token::{any, take_while},
};

struct Context {
    path: PathBuf,
    source: String,
    diagnostic: Diagnostic,
}

fn compile(path: LitStr) -> TokenStream {
    let path = path.value();
    let span = path.span();

    let base_dir = match env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|_| env::current_dir())
    {
        Ok(base_dir) => base_dir,
        Err(e) => {
            let msg = format!("Failed to get current directory: {}", e);
            return quote_spanned! { span =>  compile_error!(#msg); };
        }
    };

    let full_path = base_dir.join(path);

    let input = match fs::read_to_string(&full_path) {
        Ok(content) => content,
        Err(e) => {
            let msg = format!("Failed to read '{}': {}", full_path.display(), e);
            return quote_spanned! { span => compile_error!(#msg); };
        }
    };

    let mut tokens = input.as_str();

    let ctx = Context {
        path: full_path.to_owned(),
        source: input.to_owned(),
        diagnostic: Diagnostic::new(HashMap::from([(full_path.to_owned(), input.to_owned())])),
    };

    let (text_size, expr_defs, body) = match template(&mut tokens, &ctx).and_then(|res| {
        eof.context(StrContext::Label("end of file"))
            .parse_next(&mut tokens)
            .map(|_| res)
    }) {
        Ok(res) => res,
        Err(e) => {
            let span = Span::call_site();
            let err = e.into_inner().unwrap();
            let msg = err
                .context()
                .filter_map(|c| match c {
                    StrContext::Label(l) => Some(l.to_string()),
                    StrContext::Expected(e) => Some(format!("expected {}", e)),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join(": ");

            let msg = format!("compile error: {msg}");
            let msg_lit = syn::LitStr::new(&msg, span);

            (
                0,
                TokenStream::new(),
                quote::quote_spanned! { span =>
                    compile_error!(#msg_lit);
                },
            )
        }
    };

    let full_path_str = full_path.to_string_lossy();

    quote! {
        ::rshtml::ViewFn::new({
            let _ = include_str!(#full_path_str);

            let mut _text_size = #text_size;
            #expr_defs

            (
                move |out: &mut dyn std::fmt::Write| -> std::fmt::Result {
                    #body
                    Ok(())
                },
                _text_size
            )
        })
    }
}

pub fn rust_identifier<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1.., |c: char| c.is_alphanumeric() || c == '_')
        .verify(|s: &str| syn::parse_str::<syn::Ident>(s).is_ok())
        .parse_next(input)
}

fn component_tag_identifier<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    let start = *input;

    (
        any.verify(|c: &char| c.is_ascii_uppercase()),
        take_while(0.., |c: char| c.is_ascii_alphanumeric()),
    )
        .parse_next(input)?;

    let consumed = start.len() - input.len();

    Ok(&start[..consumed])
}
