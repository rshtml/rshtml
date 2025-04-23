mod ast_viewer;
mod config;
mod node;
mod parser;
mod viewer;

pub use node::Node;

pub fn rshtml(path: String) -> String {
    let config = config::Config::default();
    let template = std::fs::read_to_string(path).unwrap();
    let (pairs, ast) = parser::run(template.as_str(), &config).unwrap();

    //viewer::execute_pairs(pairs, 0, true);
    ast_viewer::view_node(&ast, 0);

    format!("{:?}", ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::fs;
    use crate::parser::{RsHtmlParser, Rule};
    use pest::Parser;

    #[test]
    fn test_template_format() {
        let view_name = "about.html"; //"layout.html"; //"index.html";
        let config = Config::default();
        let template = fs::read_to_string(config.views_base_path.join(view_name)).unwrap();
        let (pairs, ast) = parser::run(template.as_str(), &config).unwrap();

        //viewer::execute_pairs(pairs, 0, true);
        ast_viewer::view_node(&ast, 0);

        assert!(matches!(ast, Node::Template(_)));
    }

    // #[test]
    // fn test_template_format_without_parsing() {
    //     let template = fs::read_to_string("src/views/about.html").unwrap();
    //     match RsHtmlParser::parse(Rule::template, template.as_str()) {
    //         Ok(pairs) => {
    //             viewer::execute_pairs(pairs, 0, true);
    //         }
    //         Err(e) => {
    //             println!("Error parsing template: {:?}", e);
    //         }
    //     }
    // }
}
