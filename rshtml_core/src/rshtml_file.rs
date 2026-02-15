mod break_directive;
mod child_content_directive;
mod component;
mod continue_directive;
mod inner_text;
mod rust_block;
mod rust_stmt;
mod simple_expr;
mod simple_expr_paren;
mod template;
mod template_params;
mod text;
mod use_directive;
mod utils;

use crate::{
    context::{Context, Info},
    debug,
    diagnostic::Diagnostic,
    position::Position,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use syn::Ident;
use template::template;
use utils::{generate_fn_name, params_to_ts};
#[cfg(debug_assertions)]
use winnow::LocatingSlice;
use winnow::{
    Stateful,
    error::{StrContext, StrContextValue},
};

#[cfg(not(debug_assertions))]
pub type Input<'a, 'ctx> = Stateful<&'a str, &'a mut Context<'ctx>>;
#[cfg(debug_assertions)]
pub type Input<'a, 'ctx> = Stateful<LocatingSlice<&'a str>, &'a mut Context<'ctx>>;

pub fn compile(
    path: &Path,
    base_dir: &Path,
    struct_fields: &[String],
    extract: bool,
) -> Result<(TokenStream, TokenStream, TokenStream, Info, String), String> {
    let (full_path, mut source) = match read_template(path) {
        Ok((full_path, source)) => (full_path, source),
        Err(msg) => {
            return Err(msg);
        }
    };

    if source.starts_with('\u{FEFF}') {
        source.drain(..3);
    }

    let source_len = source.len();

    let (body, info, use_directive_scope_len) = {
        let mut ctx = Context::new(path, &source, base_dir, struct_fields);

        ctx.info.fn_name = generate_fn_name(path);

        if cfg!(debug_assertions) && extract {
            ctx.info.debug = debug::Debug::new(
                source
                    .as_bytes()
                    .iter()
                    .map(|&b| if b == b'\n' { b'\n' } else { b' ' })
                    .collect::<Vec<u8>>(),
            );
        }

        #[cfg(not(debug_assertions))]
        let mut input = Input {
            input: &source,
            state: &mut ctx,
        };
        #[cfg(debug_assertions)]
        let mut input = Input {
            input: LocatingSlice::new(&source),
            state: &mut ctx,
        };

        let body = match template(&mut input) {
            Ok(res) => res,
            Err(e) => {
                let err = e.into_inner().unwrap();

                let mut labels = Vec::new();
                let mut expecteds = Vec::new();

                for context in err.context() {
                    match context {
                        StrContext::Label(l) => labels.push(l.to_string()),
                        StrContext::Expected(e) => match e {
                            StrContextValue::Description(desc) => expecteds.push(desc.to_string()),
                            other => expecteds.push(format!("expected {}", other)),
                        },
                        _ => {}
                    }
                }

                let label = if !labels.is_empty() {
                    labels.reverse();
                    format!("in {}:", labels.join(" > "))
                } else {
                    String::new()
                };

                expecteds.reverse();
                let expected = expecteds.join(" or ");

                let offset = source.len().saturating_sub(input.input.len());
                let position: Position = (source.as_str(), offset).into();
                let diag = Diagnostic(&source).message(path, &position, &label, &expected, 1);

                return Err(diag);
            }
        };

        if cfg!(debug_assertions) && extract {
            ctx.info.debug.extract();
        }

        (body, ctx.info, source_len - ctx.last_use_directive_point)
    };

    let full_path_str = full_path.to_string_lossy();

    let mut params: Vec<(&str, &str)> = info
        .template_params
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();

    let args = params_to_ts(&mut params);
    let fn_name = Ident::new(&info.fn_name, Span::call_site());

    source.truncate(use_directive_scope_len);

    Ok((
        quote! {
         fn #fn_name(&self,
                __out__: &mut dyn ::rshtml::Write,
                child_content: impl Fn(&mut dyn ::rshtml::Write) -> ::std::fmt::Result,
                #args) -> ::std::fmt::Result;
        },
        quote! {
        fn #fn_name(&self,
                __out__: &mut dyn ::rshtml::Write,
                child_content: impl Fn(&mut dyn ::rshtml::Write) -> ::std::fmt::Result,
                #args) -> ::std::fmt::Result {#body Ok(())}
        },
        quote! { let _ = include_str!(#full_path_str); },
        info,
        source,
    ))
}

pub fn read_template(path: &Path) -> Result<(PathBuf, String), String> {
    let base_dir = match env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|_| env::current_dir())
    {
        Ok(base_dir) => base_dir,
        Err(e) => return Err(format!("Failed to get current directory: {}", e)),
    };

    let full_path = base_dir.join(path);

    let source = match fs::read_to_string(&full_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read '{}': {}", full_path.display(), e)),
    };

    Ok((full_path, source))
}
