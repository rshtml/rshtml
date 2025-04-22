mod ast_viewer;
mod config;
mod parser;
mod viewer;

pub use parser::Node;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::fs;

    #[test]
    fn test_template_format() {
        let config = Config::default();
        let template = fs::read_to_string(config.views_base_path.join("index.html")).unwrap();
        let (pairs, ast) = parser::run(template.as_str(), &config).unwrap();

        //viewer::execute_pairs(pairs, 0, true);
        ast_viewer::view_node(&ast, 0);

        assert!(matches!(ast, Node::Template(_)));
    }

    // #[test]
    // fn test_template_format2() {
    //     let template = fs::read_to_string("src/views/index.html").unwrap();
    //
    //     match TemplateParser::parse(Rule::template, template.as_str()) {
    //         Ok(pairs) => {
    //             viewer::execute_pairs(pairs, 0, true);
    //         }
    //         Err(e) => println!("Parse Error:\n{}", e),
    //     }
    // }
}
