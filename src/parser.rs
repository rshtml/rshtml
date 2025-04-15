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
        // @if ...  { ... } else { ... } / @for ... { ... }
        clauses: Vec<(String, Vec<Node>)>,
        //head: String,    // if myVar / for i in items (with trim)
        //body: Vec<Node>, // inner nodes (inner_template)
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
            let mut inner_pairs = pair.into_inner().peekable();
            let mut clauses: Vec<(String, Vec<Node>)> = Vec::new();

            // Loop through all head-body pairs captured by the (...)+ structure in pest
            while inner_pairs.peek().is_some() {
                // Skip any leading whitespace captured before the head
                while let Some(p) = inner_pairs.peek() {
                    if p.as_rule() == Rule::WHITESPACE {
                        inner_pairs.next(); // Consume whitespace
                    } else {
                        break;
                    }
                }

                // Expect a head (e.g., "if condition", "else if condition", "else", "for item in items")
                let head_pair = inner_pairs
                    .next_if(|p| p.as_rule() == Rule::rust_expr_head)
                    .ok_or_else(|| {
                        format!(
                            "Internal Error: rust_expr expected a head, found {:?}",
                            inner_pairs.peek().map(|p| p.as_rule())
                        )
                    })?;
                let head = head_pair.as_str().trim().to_string();

                // Skip any whitespace captured between the head and the opening brace '{'
                while let Some(p) = inner_pairs.peek() {
                    if p.as_rule() == Rule::WHITESPACE {
                        inner_pairs.next(); // Consume whitespace
                    } else {
                        break;
                    }
                }

                // Expect the body (inner_template)
                let template_pair = inner_pairs
                    .next_if(|p| p.as_rule() == Rule::inner_template)
                    .ok_or_else(|| {
                        format!(
                            "Internal Error: rust_expr missing inner_template for head: '{}'",
                            head
                        )
                    })?;

                // Recursively parse the nodes within the body
                let body_nodes = build_nodes_from_pairs(template_pair.into_inner())?;

                clauses.push((head.clone(), body_nodes)); // Clone head here

                // Skip any trailing whitespace captured after the closing brace '}'
                while let Some(p) = inner_pairs.peek() {
                    if p.as_rule() == Rule::WHITESPACE {
                        inner_pairs.next(); // Consume whitespace
                    } else {
                        break;
                    }
                }
            }

            if clauses.is_empty() {
                return Err("Internal Error: rust_expr parsed with no clauses".to_string());
            }

            // Basic validation for else/else if order (can be enhanced)
            let mut found_else = false;
            let first_head = clauses[0].0.split_whitespace().next().unwrap_or("");
            let is_conditional = ["if", "match"].contains(&first_head); // Assuming match also starts a chain

            for (i, (head_str, _)) in clauses.iter().enumerate() {
                let current_clause_type = head_str.split_whitespace().next().unwrap_or("");

                if current_clause_type == "else" {
                    if !is_conditional {
                        return Err(format!(
                            "Syntax Error: 'else' clause used without a preceding 'if' or 'match' (found '{}').",
                            first_head
                        ));
                    }
                    if i == 0 {
                        return Err(
                            "Syntax Error: Expression cannot start with 'else'.".to_string()
                        );
                    }
                    if i != clauses.len() - 1 {
                        return Err(
                            "Syntax Error: 'else' clause must be the last clause in the chain."
                                .to_string(),
                        );
                    }
                    found_else = true;
                } else if current_clause_type == "else if" {
                    if !is_conditional {
                        return Err(format!(
                            "Syntax Error: 'else if' clause used without a preceding 'if' or 'match' (found '{}').",
                            first_head
                        ));
                    }
                    if found_else {
                        return Err(
                            "Syntax Error: 'else if' clause found after 'else' clause.".to_string()
                        );
                    }
                    if i == 0 {
                        return Err(
                            "Syntax Error: Expression cannot start with 'else if'.".to_string()
                        );
                    }
                } else if i > 0 {
                    // If it's not the first clause
                    // Check if it's a valid start of a new expression or an invalid continuation
                    if is_conditional {
                        // If the chain started with if/match, subsequent clauses must be else/else if
                        return Err(format!(
                            "Syntax Error: Unexpected start of a new clause type '{}' within an if/match chain. Expected 'else if' or 'else'.",
                            current_clause_type
                        ));
                    }
                    // If it wasn't conditional (e.g. for/while), multiple clauses are likely an error anyway
                    return Err(format!(
                        "Syntax Error: Unexpected multiple clauses starting with '{}' and '{}'. Only one clause expected for non-conditional expressions.",
                        first_head, current_clause_type
                    ));
                } else if !["if", "for", "while", "match", "let"].contains(&current_clause_type) {
                    // Validate the start of the first clause if it's not a known keyword
                    // This might be too strict depending on allowed expressions
                    return Err(format!(
                        "Syntax Error: Expression started with unexpected keyword or construct: '{}'",
                        current_clause_type
                    ));
                }
            }

            Ok(Node::RustExpr { clauses })
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
