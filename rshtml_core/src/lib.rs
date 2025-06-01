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
    let (views_base_path, layout) = config.views.clone();

    let (compiled_ast_tokens, sections, text_size) = match parse_and_compile(&template_name, config) {
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

    let text_size = text_size + ((text_size as f64 * 0.10) as usize).max(32).min(512);

    //dbg!("DEBUG: Generated write_calls TokenStream:\n{}", compiled_ast_tokens.to_string());

    let rs = quote! {
        #[allow(non_upper_case_globals)]
        const layout: &str = #layout;
        fn has_section(section: &str) -> bool {#sections.contains(&section)}
        #[allow(unused_imports)]
        use rshtml::functions::*;
    };

    let generated_code = quote! {
        const _ : () = {

            #rs

            impl rshtml::traits::RsHtml for #struct_name {
                fn fmt(&mut self, __f__: &mut dyn ::std::fmt::Write) -> ::std::fmt::Result {

                    #compiled_ast_tokens

                    Ok(())
                }

                fn render(&mut self) -> String {
                    let mut buf = String::with_capacity(#text_size);
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

fn parse_and_compile(template_path: &str, config: Config) -> Result<(TokenStream, TokenStream, usize)> {
    let mut rshtml_parser = RsHtmlParser::new();
    let node = rshtml_parser.run(template_path, config)?;

    let mut compiler = compiler::Compiler::new();
    let ts = compiler.compile(&node)?;

    if let Some(layout) = compiler.layout.clone() {
        compiler.section_body = Some(ts.clone());
        let layout_ts = compiler.compile(&layout)?;

        return Ok((layout_ts, compiler.section_names(), compiler.text_size));
    }

    Ok((ts, compiler.section_names(), compiler.text_size))
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
