mod parser;
mod viewer;

pub use parser::{Node, parse_template};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Rule, TemplateParser};
    use pest::Parser;
    use std::fs;

    #[test]
    fn test_template_format() {
        let template = fs::read_to_string("src/views/index.html").unwrap();
        let (pairs, ast) = parse_template(template.as_str()).unwrap();

        viewer::execute_pairs(pairs, 0, true);

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
