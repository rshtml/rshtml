use crate::node::{ComponentParameterValue, Node, SectionDirectiveContent};

fn print_indent(indent: usize) {
    print!("{}", "  ".repeat(indent));
}

pub fn view_node(node: &Node, indent: usize) {
    print_indent(indent);
    match node {
        Node::Template(_, nodes, _) => {
            println!("- Template:");
            for inner_node in nodes {
                view_node(inner_node, indent + 1);
            }
        }
        Node::Text(text) => {
            println!("- Text: {text:?}");
        }
        Node::InnerText(text) => {
            println!("- InnerText: {text:?}");
        }
        Node::Comment(comment) => {
            println!("- Comment: {comment:?}");
        }
        Node::IncludeDirective(path, template) => {
            println!("- IncludeDirective: {path:?}");
            view_node(template, indent + 1);
        }
        Node::ExtendsDirective(path, _) => {
            println!("- ExtendsDirective: {path:?}");
        }
        Node::RenderDirective(path) => {
            println!("- RenderDirective: {path:?}");
        }
        Node::RustBlock(content, _) => {
            println!("- RustBlock: {content:?}");
        }
        Node::RustExprSimple(expr, _, _) => {
            println!("- RustExprSimple: {expr:?}");
        }
        Node::RustExprParen(expr, _, _) => {
            println!("- RustExprParen: {expr:?}");
        }
        Node::RustExpr(clauses, _) => {
            println!("- RustExpr:");
            for (condition, nodes) in clauses {
                print_indent(indent + 1);
                println!("- Clause: {condition:?}");
                for inner_node in nodes {
                    view_node(inner_node, indent + 2);
                }
            }
        }
        Node::MatchExpr(head, arms, _) => {
            println!("- MatchExpr:");
            print_indent(indent + 1);
            println!("- Clause: {head:?}");
            print_indent(indent + 1);
            println!("- Arms:");
            for (head, values) in arms {
                print_indent(indent + 2);
                println!("- Arm: {head:?}");
                for inner_node in values {
                    view_node(inner_node, indent + 3);
                }
            }
        }
        Node::SectionDirective(name, body, _) => {
            println!("- SectionDirective:");
            print_indent(indent + 1);
            println!("- StringLine: {name:?}");
            print_indent(indent + 1);
            match body {
                SectionDirectiveContent::Text(s) => println!("- StringLine: {s:?}"),
                SectionDirectiveContent::RustExprSimple(s, _) => {
                    println!("- RustExprSimple: {s:?}")
                }
            }
        }
        Node::SectionBlock(section_head, body, _) => {
            println!("- SectionBlock:");
            print_indent(indent + 1);
            println!("- StringLine: {section_head:?}");
            for inner_node in body {
                view_node(inner_node, indent + 1);
            }
        }
        Node::RenderBody => {
            println!("- RenderBody");
        }
        Node::Component(name, parameters, body, _) => {
            println!("- Component:");
            print_indent(indent + 1);
            println!("- Name: {name:?}");
            print_indent(indent + 1);
            println!("- Parameters:");
            for parameter in parameters {
                print_indent(indent + 2);
                println!("- Name: {:?}", parameter.name);
                print_indent(indent + 2);
                match &parameter.value {
                    ComponentParameterValue::Bool(b) => println!("- Bool: {b:?}"),
                    ComponentParameterValue::Number(b) => println!("- Number: {b:?}"),
                    ComponentParameterValue::String(s) => println!("- String: {s:?}"),
                    ComponentParameterValue::RustExprSimple(s) => {
                        println!("- RustExprSimple: {s:?}")
                    }
                    ComponentParameterValue::RustExprParen(s) => {
                        println!("- RustExprParen: {s:?}")
                    }
                    ComponentParameterValue::Block(nodes) => {
                        println!("- Block:");
                        for node in nodes {
                            view_node(node, indent + 3)
                        }
                    }
                }
            }
            for inner_node in body {
                view_node(inner_node, indent + 1);
            }
        }
        Node::ChildContent => {
            println!("- ChildContent");
        }
        Node::Raw(s) => println!("- Raw: {s:?}"),
        Node::UseDirective(component_name, import_path, component, _) => {
            println!("- UseDirective:");
            print_indent(indent + 1);
            println!("- ComponentName: {component_name:?}");
            print_indent(indent + 1);
            println!("- ImportPath: {import_path:#?}");
            print_indent(indent + 1);
            println!("- Component:");
            view_node(component, indent + 2);
        }
        Node::ContinueDirective => {
            println!("- ContinueDirective");
        }
        Node::BreakDirective => {
            println!("- BreakDirective");
        }
    }
}
