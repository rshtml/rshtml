use crate::parser::Node;

pub fn view_node(node: &Node, indent_level: usize) {
    let indent = "  ".repeat(indent_level);
    let prefix = format!("{}- ", indent);

    match node {
        Node::Template(children) => {
            println!("{}template", prefix);
            for child in children {
                view_node(child, indent_level + 1); //increase indent level for children
            }
        }
        Node::Text(content) => {
            println!("{}text: \"{}\"", prefix, content.escape_debug());
        }
        Node::InnerText(content) => {
            println!("{}inner_text: \"{}\"", prefix, content.escape_debug());
        }
        Node::Comment(content) => {
            println!(
                "{}comment_block > comment_content: \"{}\"",
                prefix,
                content.escape_debug()
            );
        }
        Node::RustBlock(content) => {
            println!(
                "{}block > rust_block: \"{}\"",
                prefix,
                content.escape_debug()
            );
        }
        Node::RustExprSimple(content) => {
            println!(
                "{}block > rust_expr_simple: \"{}\"",
                prefix,
                content.escape_debug()
            );
        }
        Node::RustExpr { head, body } => {
            println!("{}block > rust_expr", prefix);
            let child_indent = "  ".repeat(indent_level + 1);
            let child_prefix = format!("{}- ", child_indent);
            println!(
                "{}rust_expr_head: \"{}\"",
                child_prefix,
                head.escape_debug()
            );
            println!("{}inner_template", child_prefix);
            for child in body {
                view_node(child, indent_level + 2);
            }
        }
    }
}
