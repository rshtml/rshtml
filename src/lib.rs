mod node_viewer;
mod parser;

pub use node_viewer::view_node;
pub use parser::{Node, parse_template};

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_template_format() {
        let template = fs::read_to_string("src/views/index.html").unwrap();
        let ast = parse_template(template.as_str()).unwrap();

        println!("Viewing AST:");
        view_node(&ast, 0);

        assert!(matches!(ast, Node::Template(_)));
    }
}
