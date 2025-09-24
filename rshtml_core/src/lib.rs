#![doc(hidden)]

mod compiler;
pub mod config;
mod error;
mod node;
mod parser;
mod position;
pub mod str_extensions;
mod temporary_file;
#[cfg(test)]
mod tests;

use crate::config::Config;
use crate::parser::RsHtmlParser;
use crate::position::Position;
use anyhow::Result;
use node::Node;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use std::clone::Clone;
use syn::Generics;

pub fn process_template(
    template_name: String,
    struct_name: &Ident,
    struct_generics: &Generics,
) -> TokenStream {
    let config = Config::load_from_toml_or_default();
    let layout = config.layout.clone();
    let extract_file_on_debug = config.extract_file_on_debug;

    let (compiled_ast_tokens, sections, text_size) = match parse_and_compile(&template_name, config)
    {
        Ok(tokens) => tokens,
        Err(err) => {
            let error_message = format!(
                "Template processing failed for struct `{struct_name}` with template `{template_name}`:\n{err}"
            );

            return quote_spanned! { struct_name.span() => compile_error!(#error_message); };
        }
    };

    let text_size = text_size + ((text_size as f64 * 0.10) as usize).clamp(32, 512);

    let (impl_generics, ty_generics, where_clause) = struct_generics.split_for_impl();

    //dbg!("DEBUG: Generated write_calls TokenStream:\n{}", compiled_ast_tokens.to_string());

    let rs = quote! {
        #[allow(non_upper_case_globals)]
        const layout: &str = #layout;
        fn has_section(section: &str) -> bool {#sections.contains(&section)}
        #[allow(unused_imports)]
        use ::std::fmt::Write;
    };

    let generated_code = quote! {
        const _ : () = {

            #rs

            impl #impl_generics rshtml::traits::RsHtml for #struct_name #ty_generics #where_clause {
                fn fmt(&mut self, __f__: &mut dyn ::std::fmt::Write) -> ::std::fmt::Result {

                    #compiled_ast_tokens

                    Ok(())
                }

                fn render(&mut self) -> Result<String, ::std::fmt::Error> {
                    let mut buf = String::with_capacity(#text_size);
                    self.fmt(&mut buf)?;
                    Ok(buf)
                }
            }
        };
    };

    if cfg!(debug_assertions) && extract_file_on_debug {
        match temporary_file::create(&struct_name.to_string(), &generated_code.to_string()) {
            Ok(code) => return code,
            Err(err) => {
                dbg!("Failed to create temporary file:", err);
            }
        }
    }

    generated_code
}

fn parse_and_compile(
    template_path: &str,
    config: Config,
) -> Result<(TokenStream, TokenStream, usize)> {
    let mut rshtml_parser = RsHtmlParser::new();
    let node = rshtml_parser.run(template_path, config)?;

    let mut compiler = compiler::Compiler::new();
    let ts = compiler.compile(&node)?;

    if let Some(layout) = compiler.layout.clone() {
        compiler.section_body = Some(ts.clone());
        compiler
            .files
            .push((template_path.to_string(), Position::default()));
        let layout_ts = compiler.compile(&layout)?;

        return Ok((layout_ts, compiler.section_names(), compiler.text_size));
    }

    Ok((ts, compiler.section_names(), compiler.text_size))
}
