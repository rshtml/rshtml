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

use crate::{context::Context, diagnostic::Diagnostic, position::Position};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use syn::Ident;
use template::template;
use utils::{generate_fn_name, params_to_ts};
use winnow::{
    Stateful,
    error::{StrContext, StrContextValue},
};

// TODO: process here all analyzer controls
// TODO: Consider whether the import paths in the `use` statement should start from the location of the file.

pub type Input<'a> = Stateful<&'a str, &'a mut Context>;

pub fn compile<'a>(
    path: &Path,
    mut ctx: Context,
) -> Result<(TokenStream, TokenStream, TokenStream, Context), String> {
    let (full_path, source) = match read_template(path) {
        Ok((full_path, source)) => (full_path, source),
        Err(msg) => {
            return Err(msg);
        }
    };

    let body = {
        ctx.fn_name = generate_fn_name(path);

        let mut input = Input {
            input: source.as_str(),
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
                let position: Position = (source.as_str(), offset).into();
                let diag = Diagnostic(&source).message(path, &position, "this is title", &msg, 1);

                return Err(diag);
            }
        };

        body
    };

    let full_path_str = full_path.to_string_lossy();

    let mut params: Vec<(&str, &str)> = ctx
        .template_params
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();

    let args = params_to_ts(&mut params);
    let fn_name = Ident::new(&ctx.fn_name, Span::call_site());

    Ok((
        quote! {
         fn #fn_name(&self,
                __f__: &mut dyn ::std::fmt::Write,
                child_content: impl Fn(&mut dyn ::std::fmt::Write) -> ::std::fmt::Result,
                #args) -> ::std::fmt::Result;
        },
        quote! {
        fn #fn_name(&self,
                __f__: &mut dyn ::std::fmt::Write,
                child_content: impl Fn(&mut dyn ::std::fmt::Write) -> ::std::fmt::Result,
                #args) -> ::std::fmt::Result {#body Ok(())}
        },
        quote! { let _ = include_str!(#full_path_str); },
        ctx,
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
