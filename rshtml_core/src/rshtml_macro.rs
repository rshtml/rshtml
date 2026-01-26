mod extensions;
mod inner_text;
mod rust_block;
mod template;
mod template_params;
mod text;

use crate::diagnostic::Diagnostic;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use quote::quote_spanned;
use std::{collections::HashMap, env, fs, path::PathBuf};
use syn::{LitStr, spanned::Spanned};
use template::template;
use winnow::Stateful;
use winnow::error::AddContext;
use winnow::error::ParserError;
use winnow::stream::Stream;
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

        self.source = input.clone();
        // let tokens = self.source.as_str();
        let tokens = input.as_str();

        // let ctx = Context {
        //     path: full_path.to_owned(),
        //     source: input.to_owned(),
        //     diagnostic: Diagnostic::new(HashMap::from([(full_path.to_owned(), input.to_owned())])),
        //     text_size: -1,
        //     template_params: Vec::new(),
        // };

        let source_len = self.source.len();

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

                println!("source len:{source_len}, {}", input.input.len());

                let offset = source_len.saturating_sub(input.input.len());
                let diag = self.show_error(offset, &format!("{msg:?}"));

                // let msg = format!("compile error: {msg}");
                let msg_lit = syn::LitStr::new(&diag, Span::call_site());

                quote::quote_spanned! { span =>
                    compile_error!(#msg_lit);
                }
            }
        };

        let full_path_str = full_path.to_string_lossy();

        quote! {
            let _ = include_str!(#full_path_str);

            #body
        }
    }

    fn line_col(&self, byte_offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;

        for (i, ch) in self.source.char_indices() {
            if i >= byte_offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }

    fn show_error(&self, byte_offset: usize, msg: &str) -> String {
        let (line, col) = self.line_col(byte_offset);

        // ilgili satırı bul
        let line_start = self.source[..byte_offset]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);
        let line_end = self.source[byte_offset..]
            .find('\n')
            .map(|i| byte_offset + i)
            .unwrap_or(self.source.len());

        let line_text = &self.source[line_start..line_end];

        let mut caret = String::new();
        caret.push_str(&" ".repeat(col.saturating_sub(1)));
        caret.push('^');

        format!("{msg}\n --> line {line}, col {col}\n{line_text}\n{caret}")
    }
}

#[test]
fn test_rshtml_macro() {
    let path = LitStr::new("views/rshtml_macro.rs.html", Span::call_site());
    let mut rshtml_macro = RshtmlMacro::new();
    let result = rshtml_macro.compile(path);

    println!("{result}, {0:?}", rshtml_macro.template_params);
}
