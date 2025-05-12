#![allow(unused_imports, dead_code)]

use rshtml::config::Config;
use rshtml::parser;
use rshtml_macro::RsHtml;

#[derive(Debug, RsHtml)]
//#[rshtml(path = "about.rs.html")]
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

impl HomePage {
    fn my_func(&self) -> String {
        format!("{} {}", self.abc, self.def)
    }

    fn get_header(&self, title: &str) -> String {
        format!("Header: {}", title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use pest::Parser;
    use rshtml::parser::{RsHtmlParser, Rule};
    use rshtml::{ast_viewer, node::Node, process_template, viewer};
    use std::fs;
    use syn::__private::Span;

    #[test]
    fn test_macro() {
        let homepage = HomePage {
            title: "Hello".to_string(),
            content: "World".to_string(),
            card_count: 1,
            my_var: "This is my var".to_string(),
            users: vec!["John".to_string(), "Jane".to_string()],
            abc: "abc".to_string(),
            def: "def".to_string(),
            inner: "inner".to_string(),
            hey: "hey".to_string(),
            is_ok: true,
        };

        let s = homepage.to_string();

        print!("{}", s);
    }

    #[test]
    pub fn test_process() {
        let ident = syn::Ident::new("HomePage", Span::call_site());
        process_template("home.rs.html".to_string(), &ident);
    }
}
