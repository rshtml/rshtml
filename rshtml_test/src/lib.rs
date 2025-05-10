#![allow(unused_imports, dead_code)]

mod impl_display_test;

use rshtml::config::Config;
use rshtml::parser;
use rshtml_macro::RsHtml;

#[derive(Debug, RsHtml)]
//#[rshtml(path = "cards.rs.html")]
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
    use rshtml::parser::{RsHtmlParser, Rule};
    use rshtml::{Node, ast_viewer, viewer};
    use std::fs;

    #[test]
    fn test_template_format() {
        let views = vec!["layout.rs.html", "index.rs.html", "about.rs.html", "home.rs.html", "header.rs.html", "bar.rs.html"];
        let view_name = views[4];
        let config = Config::default();
        let template = fs::read_to_string(config.views_base_path.join(view_name)).unwrap();
        let (_pairs, ast) = parser::run(template.as_str(), config).unwrap();

        //viewer::execute_pairs(pairs, 0, true);
        ast_viewer::view_node(&ast, 0);

        assert!(matches!(ast, Node::Template(_)));
    }

    #[test]
    fn test_template_format_without_parsing() {
        let template = fs::read_to_string("src/views/about.rs.html").unwrap();
        rshtml::parse_without_ast(template);
    }

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
}
