#![allow(unused_variables, unused_imports)]

pub mod ast_viewer;
pub mod config;
pub mod node;
pub mod parser;
pub mod viewer;
pub mod compiler;

use crate::config::Config;
use crate::parser::{RsHtmlParser, Rule};
pub use node::Node;
use pest::Parser;
use std::path::PathBuf;
use proc_macro2::TokenStream;

pub fn parse_and_compile_ast(template_path: &str, config: Config) -> TokenStream {
    let node = parse(template_path, config);
    let mut compiler = compiler::Compiler::new();
    let ts = compiler.compile(&node);

    if let Some(layout) = compiler.layout.clone() {
        compiler.section_body = Some(ts.clone());
        let layout_ts = compiler.compile(&layout);

        return layout_ts;
    }

    ts
}

pub fn parse(path: &str, config: Config) -> Node {
    let mut base_path = PathBuf::from(&config.views_base_path);
    base_path.push(path);

    let template = std::fs::read_to_string(base_path)
        .unwrap_or_else(|err| panic!("Error reading template: {:?}, path: {}", err, path));

    parser::start_parser(&template, config)
        .unwrap_or_else(|err| panic!("Error parsing template: {:?}", err))
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
