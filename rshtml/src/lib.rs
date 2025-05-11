pub mod ast_viewer;
pub mod compiler;
pub mod config;
pub mod node;
pub mod parser;
pub mod viewer;

use crate::config::Config;
use crate::parser::{RsHtmlParser, Rule};
use eyre::{Result, eyre};
pub use node::Node;
use pest::Parser;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use std::path::PathBuf;

pub fn process_template(template_name: String, struct_name: &Ident) -> TokenStream {
    let config = Config::load_from_toml_or_default();
    let compiled_ast_tokens = match parse_and_compile(&template_name, config) {
        Ok(tokens) => tokens,
        Err(report) => {
            let error_message = format!(
                "RsHtml template processing failed for struct `{}` with template `{}`:\n{}",
                struct_name, template_name, report
            );

            return quote_spanned! {
                struct_name.span() => compile_error!(#error_message);
            }
            .into();
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

pub fn parse_and_compile(template_path: &str, config: Config) -> Result<TokenStream> {
    let node = parse(template_path, config)?;
    let mut compiler = compiler::Compiler::new();
    let ts = compiler.compile(&node);

    if let Some(layout) = compiler.layout.clone() {
        compiler.section_body = Some(ts.clone());
        let layout_ts = compiler.compile(&layout);

        return Ok(layout_ts);
    }

    Ok(ts)
}

pub fn parse(path: &str, config: Config) -> Result<Node> {
    let mut base_path = PathBuf::from(&config.views_base_path);
    base_path.push(path);

    let template = std::fs::read_to_string(base_path).map_err(|err| eyre!("Error reading template: {:?}, path: {}", err, path))?;

    parser::start_parser(&template, config).map_err(|err| eyre!("Error parsing template: {:?}", err))
}

pub fn parse_without_ast(template: String) {
    match RsHtmlParser::parse(Rule::template, template.as_str()) {
        Ok(pairs) => {
            viewer::execute_pairs(pairs, 0, true);
        }
        Err(e) => {
            println!("Error parsing template: {:?}", e);
        }
    }
}

pub fn rshtml(path: String) -> String {
    let config = config::Config::default();
    let mut base_path = PathBuf::from(&config.views_base_path);
    base_path.push(path);

    let template = std::fs::read_to_string(&base_path).unwrap();
    let (pairs, ast) = parser::run(template.as_str(), config).unwrap();

    viewer::execute_pairs(pairs, 0, true);
    ast_viewer::view_node(&ast, 0);

    format!("{:?}", ast)
}
