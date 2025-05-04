mod ast_viewer;
mod config;
mod node;
mod parser;
mod viewer;

pub use node::Node;
use rshtml_macro::RsHtml;

pub fn rshtml(path: String) -> String {
    let config = config::Config::default();
    let template = std::fs::read_to_string(path).unwrap();
    let (pairs, ast) = parser::run(template.as_str(), config).unwrap();

    viewer::execute_pairs(pairs, 0, true);
    ast_viewer::view_node(&ast, 0);

    format!("{:?}", ast)
}

#[derive(RsHtml)]
#[rshtml(path = "home.rs.html")]
struct HomePage {
    title: String,
    content: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::parser::{RsHtmlParser, Rule};
    use pest::Parser;
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
        match RsHtmlParser::parse(Rule::template, template.as_str()) {
            Ok(pairs) => {
                viewer::execute_pairs(pairs, 0, true);
            }
            Err(e) => {
                println!("Error parsing template: {:?}", e);
            }
        }
    }

    #[test]
    fn test_macro() {
        let homepage = HomePage {
            title: "Hello".to_string(),
            content: "World".to_string(),
        };

        let s = homepage.to_string();

        print!("{}", s);
    }
}
