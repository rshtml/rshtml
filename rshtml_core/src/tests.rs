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

#[test]
pub fn test_process() -> std::io::Result<()> {
    let ident = syn::Ident::new("HomePage", Span::call_site());
    let ts = process_template("home.rs.html".to_string(), &ident);

    let test_code_str = quote! {
        struct HomePage {
            title: String,
            content: String,
            card_count: usize,
            my_var: String,
            users: Vec<String>,
            abc: String,
            def: String,
            inner: String,
            hey: String,
            is_ok: bool,
        }

        #ts

        fn main() {
            let home_page = HomePage {
                title: "Home Page".to_string(),
                content: "This is the home page".to_string(),
                card_count: 10,
                my_var: "Hello".to_string(),
                users: vec!["Alice".to_string(), "Bob".to_string()],
                abc: "123".to_string(),
                def: "456".to_string(),
                inner: "789".to_string(),
                hey: "012".to_string(),
                is_ok: true,
            };

            println!("{}", home_page.to_string());
        }
    }
    .to_string();

    let dir = tempdir()?;
    let file_path = dir.path().join("test.rs");
    let mut file = File::create(&file_path)?;
    writeln!(file, "{}", test_code_str)?;

    let t = trybuild::TestCases::new();
    t.pass(&file_path);

    Ok(())
}
