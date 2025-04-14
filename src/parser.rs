use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "template.pest"]
pub struct TemplateParser;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Template(Vec<Node>),    //main template, contains child nodes
    Text(String),           // plain text content (@@ -> @)
    InnerText(String),      // text inside a block (@@ -> @, @{ -> {, @} -> })
    Comment(String),        // comment content
    RustBlock(String),      // @{ ... } block content (with trim)
    RustExprSimple(String), // @expr ... (simple expression)
    RustExpr {
        // @if ... { ... } / @for ... { ... }
        head: String,    // if myVar / for i in items (with trim)
        body: Vec<Node>, // inner nodes (inner_template)
    },
}

/// takes a list of 'Pair's (parsed nodes) and builds AST nodes from them.
fn build_nodes_from_pairs(pairs: Pairs<Rule>) -> Result<Vec<Node>, String> {
    let mut nodes = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::comment_block | Rule::block | Rule::text | Rule::inner_text => {
                nodes.push(build_ast_node(pair)?);
            }
            // skip other rules (EOI, WHITESPACE, etc.)
            _ => {}
        }
    }
    Ok(nodes)
}

/// takes a 'Pair' (parsed node) and converts it to an AST 'Node'.
fn build_ast_node(pair: Pair<Rule>) -> Result<Node, String> {
    match pair.as_rule() {
        Rule::template => {
            let children = build_nodes_from_pairs(pair.into_inner())?;
            Ok(Node::Template(children))
        }
        Rule::text => {
            let processed_text = pair.as_str().replace("@@", "@");
            Ok(Node::Text(processed_text))
        }
        Rule::inner_text => {
            let processed_text = pair
                .as_str()
                .replace("@@", "@")
                .replace("@{", "{")
                .replace("@}", "}");
            Ok(Node::InnerText(processed_text))
        }
        Rule::comment_block => {
            let content = pair
                .into_inner()
                .find(|p| p.as_rule() == Rule::comment_content)
                .map(|p| p.as_str().to_string())
                .unwrap_or_default();
            Ok(Node::Comment(content))
        }
        Rule::block => {
            if let Some(inner_block_pair) = pair.into_inner().next() {
                build_ast_node(inner_block_pair)
            } else {
                Err("Internal Error: Empty block encountered".to_string())
            }
        }
        Rule::rust_block => {
            let full_block_str = pair.as_str();
            let code_content = full_block_str
                .trim_start_matches('@')
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .to_string();
            Ok(Node::RustBlock(code_content))
        }
        Rule::rust_expr_simple => Ok(Node::RustExprSimple(pair.as_str().to_string())),
        Rule::rust_expr => {
            let mut inner_pairs = pair.into_inner();

            let head_pair = inner_pairs
                .find(|p| p.as_rule() == Rule::rust_expr_head)
                .ok_or_else(|| "Internal Error: rust_expr missing head".to_string())?;
            let head = head_pair.as_str().trim().to_string();

            let template_pair = inner_pairs
                .find(|p| p.as_rule() == Rule::inner_template)
                .ok_or_else(|| "Internal Error: rust_expr missing inner_template".to_string())?;

            let body = build_nodes_from_pairs(template_pair.into_inner())?;

            Ok(Node::RustExpr { head, body })
        }

        rule => Err(format!(
            "Internal Error: Unexpected rule encountered in build_ast_node: {:?}",
            rule
        )),
    }
}

/// takes an input string and parses it into an AST.
pub fn parse_template(input: &str) -> Result<Node, String> {
    match TemplateParser::parse(Rule::template, input) {
        Ok(mut pairs) => {
            if let Some(template_pair) = pairs.next() {
                if template_pair.as_rule() == Rule::template {
                    build_ast_node(template_pair)
                } else {
                    Err(format!(
                        "Internal Error: Expected top-level rule to be 'template', found {:?}",
                        template_pair.as_rule()
                    ))
                }
            } else {
                Err("Internal Error: Parsing produced no pairs".to_string())
            }
        }
        Err(e) => Err(format!("Parse Error:\n{}", e)),
    }
}
