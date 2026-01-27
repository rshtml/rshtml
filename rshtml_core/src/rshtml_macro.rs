mod extensions;
mod inner_text;
mod rust_block;
mod template;
mod template_params;
mod text;
mod use_directive;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use std::{env, fs, path::PathBuf};
use syn::{LitStr, spanned::Spanned};
use template::template;
use winnow::{
    Stateful,
    error::{StrContext, StrContextValue},
};

pub type Input<'a> = Stateful<&'a str, &'a mut Context>;

#[derive(Debug, Default)]
pub struct Context {
    text_size: usize,
    template_params: Vec<(String, String)>,
    use_directives: Vec<(String, PathBuf)>,
}

pub fn compile(path: LitStr) -> (TokenStream, Context) {
    let path = path.value();
    let span = path.span();

    let base_dir = match env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|_| env::current_dir())
    {
        Ok(base_dir) => base_dir,
        Err(e) => {
            let msg = format!("Failed to get current directory: {}", e);
            return (
                quote_spanned! { span =>  compile_error!(#msg); },
                Context::default(),
            );
        }
    };

    let full_path = base_dir.join(path);

    let input = match fs::read_to_string(&full_path) {
        Ok(content) => content,
        Err(e) => {
            let msg = format!("Failed to read '{}': {}", full_path.display(), e);
            return (
                quote_spanned! { span => compile_error!(#msg); },
                Context::default(),
            );
        }
    };

    let source = &input;

    let mut ctx = Context {
        text_size: 0,
        template_params: Vec::new(),
        use_directives: Vec::new(),
    };

    let mut input = Input {
        input: input.as_str(),
        state: &mut ctx,
    };

    let body = match template(&mut input) {
        Ok(res) => res,
        Err(e) => {
            let err = e.into_inner().unwrap();
            let msg = err
                .context()
                .filter_map(|c| match c {
                    StrContext::Label(l) => Some(format!("in {l}")),
                    StrContext::Expected(e) => match e {
                        StrContextValue::Description(desc) => Some(desc.to_string()),
                        other => Some(format!("expected {}", other)),
                    },
                    _ => None,
                })
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join(": ");

            let offset = source.len().saturating_sub(input.input.len());
            let diag = show_error(&source, offset, &format!("{msg:?}"));

            let msg_lit = syn::LitStr::new(&diag, Span::call_site());

            quote::quote_spanned! { span =>
                compile_error!(#msg_lit);
            }
        }
    };

    let full_path_str = full_path.to_string_lossy();

    (
        quote! {
            let _ = include_str!(#full_path_str);

            #body
        },
        ctx,
    )
}

fn line_col(source: &str, byte_offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (i, ch) in source.char_indices() {
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

fn show_error(source: &str, byte_offset: usize, msg: &str) -> String {
    let (line, col) = line_col(source, byte_offset);

    let line_start = source[..byte_offset]
        .rfind('\n')
        .map(|i| i + 1)
        .unwrap_or(0);
    let line_end = source[byte_offset..]
        .find('\n')
        .map(|i| byte_offset + i)
        .unwrap_or(source.len());

    let line_text = &source[line_start..line_end];

    let mut caret = String::new();
    caret.push_str(&" ".repeat(col.saturating_sub(1)));
    caret.push('^');

    format!("{msg}\n --> line {line}, col {col}\n{line_text}\n{caret}")
}

#[test]
fn test_rshtml_macro() {
    let path = LitStr::new("views/rshtml_macro.rs.html", Span::call_site());
    let result = compile(path);

    println!(
        "{0}, {1:?} {2:?}",
        result.0, result.1.template_params, result.1.use_directives
    );
}
