mod inner_text;
mod rust_block;
mod template;
mod template_params;
mod text;

use crate::diagnostic::Diagnostic;
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use std::{collections::HashMap, env, fs, path::PathBuf};
use syn::{LitStr, spanned::Spanned};
use template::template;
use winnow::Stateful;
use winnow::{Parser, combinator::eof, error::StrContext};

#[derive(Debug)]
struct Context<'a> {
    path: PathBuf,
    source: String,
    diagnostic: Diagnostic,
    text_size: usize,
    template_params: Vec<(&'a str, &'a str)>,
}

pub type Input<'a> = Stateful<&'a str, Context<'a>>;

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

    let tokens = input.as_str();

    let ctx = Context {
        path: full_path.to_owned(),
        source: input.to_owned(),
        diagnostic: Diagnostic::new(HashMap::from([(full_path.to_owned(), input.to_owned())])),
        text_size: 0,
        template_params: Vec::new(),
    };

    let mut input = Input {
        input: tokens,
        state: ctx,
    };

    let body = match template(&mut input).and_then(|res| {
        eof.context(StrContext::Label("end of file"))
            .parse_next(&mut input)
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

            quote::quote_spanned! { span =>
                compile_error!(#msg_lit);
            }
        }
    };

    // let full_path_str = full_path.to_string_lossy();

    body
}
