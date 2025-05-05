#![allow(unused_imports)]

use rshtml::config::Config;
use rshtml::parser;
use rshtml_macro::RsHtml;

#[derive(Debug, RsHtml)]
#[rshtml(path = "home.rs.html")]
struct HomePage {
    title: String,
    content: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rshtml::parser::{RsHtmlParser, Rule};
    use rshtml::{Node, ast_viewer, viewer};
    use std::fs;

    #[test]
    fn test_template_format() {
        let views = vec!["layout.rs.html", "index.rs.html", "about.rs.html", "home.rs.html"];
        let view_name = views[1];
        let config = Config::default();
        let template = fs::read_to_string(config.views_base_path.join(view_name)).unwrap();
        let (_pairs, ast) = parser::run(template.as_str(), config).unwrap();

        //viewer::execute_pairs(pairs, 0, true);
        ast_viewer::view_node(&ast, 0);

        assert!(matches!(ast, Node::Template(_)));
    }

    #[test]
    fn test_template_format_without_parsing() {
        let template = fs::read_to_string("src/views/home.rs.html").unwrap();
        rshtml::parse_without_ast(template);
    }

    #[test]
    fn test_macro() {
        //let config = Config::default();

        //println!("{:?}", config);

        let homepage = HomePage {
            title: "Hello".to_string(),
            content: "World".to_string(),
        };

        let s = homepage.to_string();

        print!("{}", s);
    }
}
