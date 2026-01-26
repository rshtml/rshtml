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

pub type Input<'a> = Stateful<&'a str, &'a mut RshtmlMacro>;

#[derive(Debug)]
pub struct RshtmlMacro {
    path: PathBuf,
    source: String,
    diagnostic: Diagnostic,
    text_size: usize,
    pub template_params: Vec<(String, String)>,
}

// #[derive(Debug)]
// struct Context<'a> {
//     path: PathBuf,
//     source: String,
//     diagnostic: Diagnostic,
//     text_size: usize,
//     template_params: Vec<(&'a str, &'a str)>,
// }
impl RshtmlMacro {
    pub fn new() -> Self {
        RshtmlMacro {
            path: PathBuf::new(),
            source: String::new(),
            diagnostic: Diagnostic::new(HashMap::new()),
            text_size: 0,
            template_params: Vec::new(),
        }
    }

    fn compile(&mut self, path: LitStr) -> TokenStream {
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

        // self.source = input;
        // let tokens = self.source.as_str();
        let tokens = input.as_str();

        // let ctx = Context {
        //     path: full_path.to_owned(),
        //     source: input.to_owned(),
        //     diagnostic: Diagnostic::new(HashMap::from([(full_path.to_owned(), input.to_owned())])),
        //     text_size: -1,
        //     template_params: Vec::new(),
        // };

        let mut input = Input {
            input: tokens,
            state: self,
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
}

#[test]
fn test_rshtml_macro() {
    let path = LitStr::new("views/rshtml_macro.rs.html", Span::call_site());
    let mut rshtml_macro = RshtmlMacro::new();
    let result = rshtml_macro.compile(path);

    println!("{result}, {0:?}", rshtml_macro.template_params);
}
