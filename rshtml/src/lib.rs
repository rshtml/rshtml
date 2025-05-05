pub mod ast_viewer;
pub mod config;
pub mod node;
pub mod parser;
pub mod viewer;

use crate::parser::{RsHtmlParser, Rule};
pub use node::Node;
use std::path::PathBuf;
use pest::Parser;

pub fn parse(path: &str) -> Node {
    let config = config::Config::default();
    let mut base_path = PathBuf::from(&config.views_base_path);
    base_path.push(path);

    let template = std::fs::read_to_string(base_path).unwrap_or_else(|err| panic!("Error reading template: {:?}, path: {}", err, path));

    parser::start_parser(&template, config).unwrap_or_else(|err| panic!("Error parsing template: {:?}", err))
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
