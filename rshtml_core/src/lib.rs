mod compiler;
mod config;
mod error;
pub mod functions;
mod node;
mod parser;
#[cfg(test)]
mod tests;
pub mod traits;

use crate::config::Config;
use crate::parser::RsHtmlParser;
use anyhow::Result;
use node::Node;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use std::fs;
use std::path::Path;

pub fn process_template(template_name: String, struct_name: &Ident) -> TokenStream {
    let config = Config::load_from_toml_or_default();
    let (views_base_path,layout) = config.views.clone();
    let (locales_base_path, locale_lang) = config.locales.clone();
    let locales_base_path = locales_base_path.to_string_lossy().into_owned();
    

    let (compiled_ast_tokens, sections) = match parse_and_compile(&template_name, config) {
        Ok(tokens) => tokens,
        Err(err) => {
            let error_message = format!(
                "Template processing failed for struct `{}` with template `{}`:\n{}",
                struct_name,
                template_name,
                err.to_string()
            );

            return quote_spanned! { struct_name.span() => compile_error!(#error_message); }.into();
        }
    };

    //dbg!("DEBUG: Generated write_calls TokenStream:\n{}", compiled_ast_tokens.to_string());

    // TODO: calculate text size in compiler and use it in render for string capacity

    let generated_code = quote! {
        #[allow(non_upper_case_globals)]
        const _ : () = {
            static rs: ::std::sync::LazyLock<rshtml::Functions> = ::std::sync::LazyLock::new(|| rshtml::Functions::new(#layout.to_string(), #sections, #locales_base_path, #locale_lang));

            impl rshtml::traits::RsHtml for #struct_name {
                fn fmt(&mut self, __f__: &mut dyn ::std::fmt::Write) -> ::std::fmt::Result {

                    #compiled_ast_tokens

                    Ok(())
                }

                fn render(&mut self) -> String {
                    let mut buf = String::with_capacity(1024);
                    self.fmt(&mut buf).unwrap();
                    buf
                }
            }
        };
    };

    if views_base_path.is_dir() {
        walk_dir(&views_base_path);
    }

    TokenStream::from(generated_code)
}

fn parse_and_compile(template_path: &str, config: Config) -> Result<(TokenStream, TokenStream)> {
    let mut rshtml_parser = RsHtmlParser::new();
    let node = rshtml_parser.run(template_path, config)?;

    let mut compiler = compiler::Compiler::new();
    let ts = compiler.compile(&node)?;

    if let Some(layout) = compiler.layout.clone() {
        compiler.section_body = Some(ts.clone());
        let layout_ts = compiler.compile(&layout)?;

        return Ok((layout_ts, compiler.section_names()));
    }

    Ok((ts, compiler.section_names()))
}

fn walk_dir(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    walk_dir(&path);
                } else if path.is_file() {
                    if let Some(path_str) = path.to_str() {
                        println!("cargo:rerun-if-changed={}", path_str);
                    }
                }
            }
        }
    }
}
