mod parser;
mod viewer;

pub use parser::{Node, parse_template};

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_template_format() {
        let template = fs::read_to_string("src/views/index.html").unwrap();
        let (pairs, ast) = parse_template(template.as_str()).unwrap();

        viewer::execute_pairs(pairs, 0, true);

        assert!(matches!(ast, Node::Template(_)));
    }
}
