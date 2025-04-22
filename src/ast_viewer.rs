use crate::node::{Node, RustBlockContent, TextBlockItem, TextLineItem};

fn print_indent(indent: usize) {
    print!("{}", "  ".repeat(indent));
}

fn view_text_line_item(item: &TextLineItem, indent: usize) {
    print_indent(indent);
    match item {
        TextLineItem::Text(text) => println!("- Text: {:?}", text),
        TextLineItem::RustExprSimple(expr) => println!("- RustExprSimple: {:?}", expr),
    }
}

fn view_text_block_item(item: &TextBlockItem, indent: usize) {
    print_indent(indent);
    match item {
        TextBlockItem::Text(text) => println!("- Text: {:?}", text),
        TextBlockItem::RustExprSimple(expr) => println!("- RustExprSimple: {:?}", expr),
    }
}

fn view_rust_block_content(content: &RustBlockContent, indent: usize) {
    print_indent(indent);
    match content {
        RustBlockContent::Code(code) => {
            println!("- Code: {:?}", code);
        }
        RustBlockContent::TextLine(items) => {
            println!("- TextLine:");
            for item in items {
                view_text_line_item(item, indent + 1);
            }
        }
        RustBlockContent::TextBlock(items) => {
            println!("- TextBlock:");
            for item in items {
                view_text_block_item(item, indent + 1);
            }
        }
        RustBlockContent::NestedBlock(contents) => {
            println!("- NestedBlock:");
            for inner_content in contents {
                view_rust_block_content(inner_content, indent + 1);
            }
        }
    }
}

pub fn view_node(node: &Node, indent: usize) {
    print_indent(indent);
    match node {
        Node::Template(nodes) => {
            println!("- Template:");
            for inner_node in nodes {
                view_node(inner_node, indent + 1);
            }
        }
        Node::Text(text) => {
            println!("- Text: {:?}", text);
        }
        Node::InnerText(text) => {
            println!("- InnerText: {:?}", text);
        }
        Node::Comment(comment) => {
            println!("- Comment: {:?}", comment);
        }
        Node::IncludeDirective(path) => {
            println!("- IncludeDirective: {:?}", path);
        }
        Node::RustBlock(contents) => {
            println!("- RustBlock:");
            for content in contents {
                view_rust_block_content(content, indent + 1);
            }
        }
        Node::RustExprSimple(expr) => {
            println!("- RustExprSimple: {:?}", expr);
        }
        Node::RustExprParen(expr) => {
            println!("- RustExprParen: {:?}", expr);
        }
        Node::RustExpr { clauses } => {
            println!("- RustExpr:");
            for (condition, nodes) in clauses {
                print_indent(indent + 1);
                println!("- Clause: {:?}", condition);
                for inner_node in nodes {
                    view_node(inner_node, indent + 2);
                }
            }
        }
    }
}
