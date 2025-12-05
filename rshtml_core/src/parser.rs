mod block;
mod comment_block;
mod component;
mod include_directive;
mod inner_text;
mod match_expr;
mod props_directive;
mod raw_block;
mod rust_block;
mod rust_expr;
mod rust_expr_paren;
mod rust_expr_simple;
mod section_block;
mod template;
mod text;
mod use_directive;

use std::collections::HashMap;

use crate::config::Config;
use crate::error::{E, rename_rules};
use crate::node::*;
use crate::parser::block::BlockParser;
use crate::parser::comment_block::CommentBlockParser;
use crate::parser::component::ComponentParser;
use crate::parser::include_directive::IncludeDirectiveParser;
use crate::parser::inner_text::InnerTextParser;
use crate::parser::match_expr::MatchExprParser;
use crate::parser::props_directive::PropsDirectiveParser;
use crate::parser::raw_block::RawBlockParser;
use crate::parser::rust_block::RustBlockParser;
use crate::parser::rust_expr::RustExprParser;
use crate::parser::rust_expr_paren::RustExprParenParser;
use crate::parser::rust_expr_simple::RustExprSimpleParser;
use crate::parser::section_block::SectionBlockParser;
use crate::parser::template::TemplateParser;
use crate::parser::text::TextParser;
use crate::parser::use_directive::UseDirectiveParser;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::{Parser, Span};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "rshtml.pest"]
pub struct RsHtmlParser {
    config: Config,
    files: Vec<String>,
    pub sources: HashMap<String, String>,
}

impl RsHtmlParser {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            files: Vec::new(),
            sources: HashMap::new(),
        }
    }

    fn build_nodes_from_pairs(
        &mut self,
        pairs: Pairs<Rule>,
    ) -> Result<Vec<Node>, Box<Error<Rule>>> {
        let mut nodes = Vec::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::template_content => {
                    let inner_nodes = self.build_nodes_from_pairs(pair.into_inner())?;
                    nodes.extend(inner_nodes);
                }
                Rule::props_directive
                | Rule::comment_block
                | Rule::block
                | Rule::text
                | Rule::inner_text => {
                    nodes.push(self.build_ast_node(pair)?);
                }
                // skip other rules (EOI, WHITESPACE, etc.)
                _ => {}
            }
        }
        Ok(nodes)
    }

    fn build_ast_node(&mut self, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        match pair.as_rule() {
            Rule::template => TemplateParser::parse(self, pair),
            Rule::text => TextParser::parse(self, pair),
            Rule::inner_text => InnerTextParser::parse(self, pair),
            Rule::comment_block => CommentBlockParser::parse(self, pair),
            Rule::props_directive => PropsDirectiveParser::parse(self, pair),
            Rule::block => BlockParser::parse(self, pair),
            Rule::include_directive => IncludeDirectiveParser::parse(self, pair),
            Rule::rust_block => RustBlockParser::parse(self, pair),
            Rule::rust_expr_simple => RustExprSimpleParser::parse(self, pair),
            Rule::rust_expr_paren => RustExprParenParser::parse(self, pair),
            Rule::rust_expr => RustExprParser::parse(self, pair),
            Rule::match_expr => MatchExprParser::parse(self, pair),
            Rule::section_block => SectionBlockParser::parse(self, pair),
            Rule::component => ComponentParser::parse(self, pair),
            Rule::child_content_directive => Ok(Node::ChildContent),
            Rule::raw_block => RawBlockParser::parse(self, pair),
            Rule::use_directive => UseDirectiveParser::parse(self, pair),
            Rule::continue_directive => Ok(Node::ContinueDirective),
            Rule::break_directive => Ok(Node::BreakDirective),
            rule => Err(E::mes(format!("Error: Unknown rule: {rule:?}")).span(pair.as_span())),
        }
    }

    fn parse_template(&mut self, path: &str) -> Result<Node, Box<Error<Rule>>> {
        let input = self.read_template(path).map_err(|err| {
            E::mes(format!("Error reading template: {err:?}, path: {path}"))
                .span(Span::new(path, 0, 0).unwrap())
        })?;

        let mut pairs = Self::parse(Rule::template, &input)?;
        let template_pair = pairs.next().ok_or(
            E::mes("Error: Empty template").position(pest::Position::new("Template", 0).unwrap()),
        )?;

        if template_pair.as_rule() == Rule::template {
            if self.files.contains(&path.to_string()) {
                return Err(
                    E::mes(format!("Error: Circular call detected for '{path}'"))
                        .span(template_pair.as_span()),
                );
            }

            self.sources
                .entry(path.to_owned())
                .or_insert_with(|| input.clone());
            self.files.push(path.to_string());

            let ast = self.build_ast_node(template_pair)?;

            self.files.pop();

            Ok(ast)
        } else {
            Err(E::pos(Rule::template).span(template_pair.as_span()))
        }
    }

    fn read_template(&self, path: &str) -> Result<String, String> {
        let view_path = self.config.base_path.join(path);
        let template = std::fs::read_to_string(&view_path).map_err(|err| {
            format!(
                "Error reading template: {:?}, path: {}",
                err,
                view_path.to_string_lossy()
            )
        })?;

        Ok(template)
    }

    pub fn run(&mut self, path: &str, config: Config) -> Result<Node, Box<Error<Rule>>> {
        self.config = config;
        self.parse_template(path).map_err(|err| rename_rules(*err))
    }
}

pub trait IParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>>;
}
