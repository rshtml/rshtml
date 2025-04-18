use crate::parser::{Node, RustBlockContent};

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
        // Node::RustBlock(content) => {
        //     println!(
        //         "{}block > rust_block: \"{}\"",
        //         prefix,
        //         content.escape_debug()
        //     );
        // }
        Node::RustBlock(contents) => {
            println!("{}block > rust_block", prefix);
            
            for content_part in contents {
                view_rust_block_content(content_part, indent_level + 1);
            }
        }
        Node::RustExprSimple(content) => {
            println!(
                "{}block > rust_expr_simple: \"{}\"",
                prefix,
                content.escape_debug()
            );
        }
        Node::RustExprParen(content) => {
            println!(
                "{}block > rust_expr_paren: \"{}\"",
                prefix,
                content.escape_debug()
            );
        }
        Node::RustExpr { clauses } => {
            println!("{}block > rust_expr", prefix);
            let child_indent = "  ".repeat(indent_level + 1);
            let child_prefix = format!("{}- ", child_indent);
            for (head, body) in clauses {
                println!(
                    "{}rust_expr_head: \"{}\"",
                    child_prefix,
                    head.escape_debug()
                );
                println!("{}inner_template", child_prefix);
                for child in body {
                    view_node(child, indent_level + 2); // Indent further for nodes inside the clause body
                }
            }
        }
    }
}

fn view_rust_block_content(content: &RustBlockContent, indent_level: usize) {
    let indent = "  ".repeat(indent_level);
    let prefix = format!("{}- ", indent);

    match content {
        RustBlockContent::Code(code_str) => {
            println!("{}rust_code: \"{}\"", prefix, code_str.escape_debug());
        }
        RustBlockContent::TextLine(text_str) => {
            println!("{}text_line: \"{}\"", prefix, text_str.escape_debug());
        }
        RustBlockContent::NestedBlock(nested_contents) => {
            println!("{}nested_block", prefix);
            for nested_content in nested_contents {
                view_rust_block_content(nested_content, indent_level + 1);
            }
        }
    }
}
