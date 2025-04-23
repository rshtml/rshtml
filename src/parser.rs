use crate::config::Config;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use std::collections::HashSet;

use crate::node::*;

#[derive(Parser)]
#[grammar = "rshtml.pest"]
pub struct RsHtmlParser;

impl RsHtmlParser {
    fn build_nodes_from_pairs(
        &self,
        pairs: Pairs<Rule>,
        config: &Config,
        included_templates: &HashSet<String>,
    ) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::section_block
                | Rule::extends_directive
                | Rule::comment_block
                | Rule::block
                | Rule::text
                | Rule::inner_text => {
                    nodes.push(self.build_ast_node(pair, config, included_templates)?);
                }
                // skip other rules (EOI, WHITESPACE, etc.)
                _ => {}
            }
        }
        Ok(nodes)
    }

    fn build_ast_node(
        &self,
        pair: Pair<Rule>,
        config: &Config,
        included_templates: &HashSet<String>,
    ) -> Result<Node, String> {
        match pair.as_rule() {
            Rule::template => {
                let children =
                    self.build_nodes_from_pairs(pair.into_inner(), config, included_templates)?;
                Ok(Node::Template(children))
            }
            Rule::text => {
                let processed_text = pair
                    .as_str()
                    .replace("@@", "@")
                    .replace("@@{", "{")
                    .replace("@@}", "}");
                Ok(Node::Text(processed_text))
            }
            Rule::inner_text => {
                let processed_text = pair
                    .as_str()
                    .replace("@@", "@")
                    .replace("@@{", "{")
                    .replace("@@}", "}");
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
                    self.build_ast_node(inner_block_pair, config, included_templates)
                } else {
                    Err("Internal Error: Empty block encountered".to_string())
                }
            }
            Rule::include_directive => {
                let path_pair = pair
                    .into_inner()
                    .find(|p| p.as_rule() == Rule::include_path)
                    .unwrap();

                let path = path_pair
                    .as_str()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();

                let view_path = config.views_base_path.join(&path);

                let included_content = match std::fs::read_to_string(&view_path) {
                    Ok(content) => content,
                    Err(e) => {
                        return Err(format!("Error reading included file '{}': {}", path, e));
                    }
                };

                let canonical_path = view_path
                    .canonicalize()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                if included_templates.contains(&canonical_path) {
                    return Err(format!(
                        "Error: Circular include detected for file '{}'",
                        path
                    ));
                }

                let mut included_templates = included_templates.clone();
                included_templates.insert(canonical_path);

                let inner_template = self.parse_template(
                    included_content.clone().as_str(),
                    config,
                    &included_templates,
                )?;

                let nodes = match inner_template {
                    Node::Template(nodes) => nodes,
                    _ => {
                        return Err(format!(
                            "Error: Expected a template in the included file '{}', found {:?}",
                            path, inner_template
                        ));
                    }
                };

                Ok(Node::Template(nodes))
                //Ok(Node::IncludeDirective(pair.as_str().to_string()))
            }
            Rule::yield_directive => Ok(Node::YieldDirective(pair.as_str().to_string())),
            Rule::extends_directive => {
                let path_pair = pair
                    .into_inner()
                    .find(|p| p.as_rule() == Rule::include_path)
                    .unwrap();
                let path_str = path_pair
                    .as_str()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();

                Ok(Node::ExtendsDirective(path_str))
            }
            Rule::rust_block => {
                let contents = self.build_rust_block_contents(pair.into_inner())?;
                Ok(Node::RustBlock(contents))
            }
            Rule::rust_expr_simple => Ok(Node::RustExprSimple(pair.as_str().to_string())),
            Rule::rust_expr_paren => Ok(Node::RustExprParen(pair.as_str().to_string())),
            Rule::rust_expr => {
                let mut inner_pairs = pair.into_inner().peekable();
                let mut clauses: Vec<(String, Vec<Node>)> = Vec::new();

                // Loop through all head-body pairs captured by the (...)+ structure in pest
                while inner_pairs.peek().is_some() {
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
                    let body_nodes = self.build_nodes_from_pairs(
                        template_pair.into_inner(),
                        config,
                        included_templates,
                    )?;

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
                                "Syntax Error: 'else if' clause found after 'else' clause."
                                    .to_string(),
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
                    } else if !["if", "for", "while", "match", "let"].contains(&current_clause_type)
                    {
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
            Rule::section_block => {
                let name_pair = pair
                    .clone()
                    .into_inner()
                    .find(|p| p.as_rule() == Rule::include_path)
                    .unwrap();

                let name = name_pair
                    .as_str()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();

                let body =
                    self.build_nodes_from_pairs(pair.into_inner(), config, included_templates)?;
                Ok(Node::SectionBlock { name, body })
            }

            rule => Err(format!(
                "Internal Error: Unexpected rule encountered in build_ast_node: {:?}",
                rule
            )),
        }
    }

    fn parse_template(
        &self,
        input: &str,
        config: &Config,
        included_templates: &HashSet<String>,
    ) -> Result<Node, String> {
        let mut pairs = Self::parse(Rule::template, input).unwrap();
        let template_pair = pairs.next().unwrap();
        if template_pair.as_rule() == Rule::template {
            let ast = self.build_ast_node(template_pair, config, included_templates)?;
            Ok(ast)
        } else {
            panic!("Expected 'template', found {:?}", template_pair.as_rule());
        }
    }

    fn build_rust_block_contents(
        &self,
        pairs: Pairs<Rule>,
    ) -> Result<Vec<RustBlockContent>, String> {
        let mut content_parts = Vec::new();
        for inner_pair in pairs {
            match inner_pair.as_rule() {
                Rule::text_line_directive => {
                    content_parts.push(self.build_text_line(inner_pair));
                }
                Rule::text_block_tag => {
                    content_parts.push(self.build_text_block(inner_pair));
                }
                Rule::rust_code => {
                    content_parts.push(RustBlockContent::Code(inner_pair.as_str().to_string()));
                }
                Rule::nested_block => {
                    let nested_contents =
                        self.build_rust_block_contents(inner_pair.into_inner())?;
                    content_parts.push(RustBlockContent::NestedBlock(nested_contents));
                }
                rule => {
                    return Err(format!(
                        "Internal Error: Unexpected rule {:?} inside rust block content",
                        rule
                    ));
                }
            }
        }
        Ok(content_parts)
    }

    fn build_text_line(&self, inner_pair: Pair<Rule>) -> RustBlockContent {
        let mut items = Vec::new();

        for item_pair in inner_pair.into_inner() {
            match item_pair.as_rule() {
                Rule::rust_expr_simple => {
                    if let Some(expr_pair) = item_pair.into_inner().nth(1) {
                        items.push(TextLineItem::RustExprSimple(expr_pair.as_str().to_string()));
                    } else {
                        eprintln!("Warning: Empty or bad embedded expression found.");
                    }
                }
                Rule::text_line => {
                    let text = item_pair.as_str().replace("@@", "@");
                    if !text.is_empty() {
                        items.push(TextLineItem::Text(text));
                    }
                }
                _ => {
                    eprintln!(
                        "Warning: Unexpected rule in text_block: {:?}",
                        item_pair.as_rule()
                    );
                }
            }
        }

        RustBlockContent::TextLine(items)
    }

    fn build_text_block(&self, inner_pair: Pair<Rule>) -> RustBlockContent {
        let mut items = Vec::new();

        for item_pair in inner_pair.into_inner() {
            match item_pair.as_rule() {
                Rule::rust_expr_simple => {
                    if let Some(expr_pair) = item_pair.into_inner().nth(1) {
                        items.push(TextBlockItem::RustExprSimple(
                            expr_pair.as_str().to_string(),
                        ));
                    } else {
                        eprintln!("Warning: Empty or bad embedded expression found.");
                    }
                }
                Rule::text_block => {
                    let text = item_pair.as_str().replace("@@", "@");
                    if !text.is_empty() {
                        items.push(TextBlockItem::Text(text));
                    }
                }
                _ => {
                    eprintln!(
                        "Warning: Unexpected rule in text_block: {:?}",
                        item_pair.as_rule()
                    );
                }
            }
        }

        RustBlockContent::TextBlock(items)
    }
}

pub fn run<'a>(input: &'a str, config: &Config) -> Result<(Pairs<'a, Rule>, Node), String> {
    let rshtml_parser = RsHtmlParser {};
    let node = rshtml_parser.parse_template(input, config, &HashSet::new())?;
    let pairs = RsHtmlParser::parse(Rule::template, input).unwrap();

    Ok((pairs.clone(), node))
}
