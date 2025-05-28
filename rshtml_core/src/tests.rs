mod ast_viewer;
mod viewer;

use crate::config::Config;
use crate::node::Node;
use crate::parser::{RsHtmlParser, Rule};
use crate::process_template;
use pest::Parser;
use std::fs;
use syn::__private::Span;

#[test]
fn test_template_format() {
    let views = vec![
        "layout.rs.html",
        "index.rs.html",
        "about.rs.html",
        "home.rs.html",
        "header.rs.html",
        "bar.rs.html",
        "component.rs.html",
    ];

    let ast = match RsHtmlParser::new().run(views[6], Config::default()) {
        Ok(ast) => ast,
        Err(err) => {
            let message = format!("{}", err.to_string());
            println!("{}", message);
            return;
        }
    };

    ast_viewer::view_node(&ast, 0);

    assert!(matches!(ast, Node::Template(_)));
}

#[test]
fn test_template_format_without_parsing() {
    let template = fs::read_to_string("views/component.rs.html").unwrap();
    let pairs = match RsHtmlParser::parse(Rule::template, template.as_str()) {
        Ok(pairs) => pairs,
        Err(err) => {
            let message = format!("{}", err.to_string());
            println!("{}", message);
            return;
        }
    };

    viewer::execute_pairs(pairs, 0, true);
}

#[test]
pub fn test_process_simple() {
    let ident = syn::Ident::new("HomePage", Span::call_site());
    process_template("continue_break.rs.html".to_string(), &ident);
}

#[test]
pub fn test_config() {
    let config = Config::default();
    assert!(config.views.0.ends_with("views"));
    assert_eq!(config.views.1, "layout.rs.html".to_string());
}
