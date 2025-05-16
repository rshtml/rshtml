mod compiler;
mod config;
mod error;
mod node;
mod parser;
#[cfg(test)]
mod tests;

use crate::config::Config;
use crate::parser::RsHtmlParser;
use anyhow::Result;
use node::Node;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};

pub fn process_template(template_name: String, struct_name: &Ident) -> TokenStream {
    let config = Config::load_from_toml_or_default();
    let compiled_ast_tokens = match parse_and_compile(&template_name, config) {
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

    let generated_code = quote! {
        impl ::std::fmt::Display for #struct_name {
             fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {

                #compiled_ast_tokens

                Ok(())
             }
        }
    };

    if let Err(err) = syn::parse_str::<syn::ItemImpl>(&generated_code.to_string()) {
        eprintln!("`generated_code`: err to generate code: {:?}", err);
    }

    TokenStream::from(generated_code)
}

fn parse_and_compile(template_path: &str, config: Config) -> Result<TokenStream> {
    let mut rshtml_parser = RsHtmlParser::new();
    let node = rshtml_parser.run(template_path, config)?;

    let mut compiler = compiler::Compiler::new();
    let ts = compiler.compile(&node)?;

    if let Some(layout) = compiler.layout.clone() {
        compiler.section_body = Some(ts.clone());
        let layout_ts = compiler.compile(&layout)?;

        return Ok(layout_ts);
    }

    Ok(ts)
}
