use crate::Node;
use crate::config::Config;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::iterators::Pair;
use std::collections::HashSet;

pub struct RustExprParser;

impl IParser for RustExprParser {
    fn parse(parser: &RsHtmlParser, pair: Pair<Rule>, config: &Config, included_templates: &HashSet<String>) -> Result<Node, String> {
        let mut inner_pairs = pair.into_inner().peekable();
        let mut clauses: Vec<(String, Vec<Node>)> = Vec::new();

        // Loop through all head-body pairs captured by the (...)+ structure in pest
        while inner_pairs.peek().is_some() {
            // Expect a head (e.g., "if condition", "else if condition", "else", "for item in items")
            let head_pair = inner_pairs
                .next_if(|p| p.as_rule() == Rule::rust_expr_head)
                .ok_or_else(|| format!("Internal Error: rust_expr expected a head, found {:?}", inner_pairs.peek().map(|p| p.as_rule())))?;
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
                .ok_or_else(|| format!("Internal Error: rust_expr missing inner_template for head: '{}'", head))?;

            // Recursively parse the nodes within the body
            let body_nodes = parser.build_nodes_from_pairs(template_pair.into_inner(), config, included_templates)?;

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
                    return Err(format!("Syntax Error: 'else' clause used without a preceding 'if' or 'match' (found '{}').", first_head));
                }
                if i == 0 {
                    return Err("Syntax Error: Expression cannot start with 'else'.".to_string());
                }
                if i != clauses.len() - 1 {
                    return Err("Syntax Error: 'else' clause must be the last clause in the chain.".to_string());
                }
                found_else = true;
            } else if current_clause_type == "else if" {
                if !is_conditional {
                    return Err(format!("Syntax Error: 'else if' clause used without a preceding 'if' or 'match' (found '{}').", first_head));
                }
                if found_else {
                    return Err("Syntax Error: 'else if' clause found after 'else' clause.".to_string());
                }
                if i == 0 {
                    return Err("Syntax Error: Expression cannot start with 'else if'.".to_string());
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
                return Err(format!("Syntax Error: Expression started with unexpected keyword or construct: '{}'", current_clause_type));
            }
        }

        Ok(Node::RustExpr { clauses })
    }
}
