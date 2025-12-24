#![doc(hidden)]

mod analyzer;
mod compiler;
pub mod config;
mod diagnostic;
mod error;
mod node;
mod parser;
mod position;
pub mod str_extensions;
mod temporary_file;
#[cfg(test)]
mod tests;

use crate::parser::RsHtmlParser;
use crate::{config::Config, diagnostic::Diagnostic};
use anyhow::Result;
use node::Node;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::Generics;

pub fn process_template(
    template_name: String,
    struct_name: &Ident,
    struct_generics: &Generics,
    struct_fields: Vec<String>,
    no_warn: bool,
) -> TokenStream {
    let config = Config::load_from_toml_or_default();
    let extract_file_on_debug = config.extract_file_on_debug;

    let (compiled_ast_tokens, text_size, components) = match parse_and_compile(
        &template_name,
        config,
        struct_name,
        struct_generics,
        struct_fields,
        no_warn,
    ) {
        Ok(tokens) => tokens,
        Err(err) => {
            let error_message = format!(
                "Template processing failed for struct `{struct_name}` with template `{template_name}`:\n{err}"
            );

            return quote_spanned! { struct_name.span() => compile_error!(#error_message); };
        }
    };

    let text_size = text_size + ((text_size as f64 * 0.10) as usize).clamp(32, 512);

    let (impl_generics, type_generics, where_clause) = struct_generics.split_for_impl();

    // dbg!("DEBUG: Generated write_calls TokenStream:\n{}", compiled_ast_tokens.to_string());

    let rs = quote! {
        #[allow(unused_imports)]
        use ::std::fmt::{Write, Display};
        #[allow(unused_imports)]
        use ::rshtml::traits::Render;
        #[allow(unused_imports)]
        use ::rshtml::Block;
    };

    let generated_code = quote! {
        #[allow(clippy::too_many_arguments)]
        const _ : () = {

            #rs

            impl #impl_generics #struct_name #type_generics #where_clause {
                #components
            }

            impl #impl_generics ::rshtml::traits::RsHtml for #struct_name #type_generics #where_clause {
                fn fmt(&self, __f__: &mut dyn ::std::fmt::Write) -> ::std::fmt::Result {

                    #compiled_ast_tokens

                    Ok(())
                }

                fn render(&self) -> Result<String, ::std::fmt::Error> {
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
    struct_name: &Ident,
    struct_generics: &Generics,
    struct_fields: Vec<String>,
    no_warn: bool,
) -> Result<(TokenStream, usize, TokenStream)> {
    let mut rshtml_parser = RsHtmlParser::new();
    let node = rshtml_parser.run(template_path, config)?;

    let analyzer = analyzer::Analyzer::run(
        template_path.to_owned(),
        &node,
        Diagnostic::new(rshtml_parser.sources),
        struct_fields,
        no_warn,
    );

    let mut compiler = compiler::Compiler::new(
        struct_name.to_owned(),
        struct_generics.to_owned(),
        analyzer.diagnostic,
    );
    let ts = compiler.run(node)?;

    Ok((ts, compiler.text_size, compiler.component_fns()))
}
