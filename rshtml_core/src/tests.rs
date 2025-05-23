mod ast_viewer;
mod viewer;

use crate::config::Config;
use crate::node::Node;
use crate::parser::{RsHtmlParser, Rule};
use crate::process_template;
use pest::Parser;
use quote::quote;
use std::fs;
use std::fs::File;
use std::io::Write;
use syn::__private::Span;
use tempfile::tempdir;

#[test]
fn test_template_format() {
    let views = vec![
        "layout.rs.html",
        "index.rs.html",
        "about.rs.html",
        "home.rs.html",
        "header.rs.html",
        "bar.rs.html",
    ];

    let ast = match RsHtmlParser::new().run(views[3], Config::default()) {
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
    let template = fs::read_to_string("tests/views/home.rs.html").unwrap();
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
    process_template("home.rs.html".to_string(), &ident);
}
