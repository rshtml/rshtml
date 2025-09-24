mod block;
mod comment_block;
mod component;
mod component_tag;
mod extends_directive;
mod include_directive;
mod inner_text;
mod match_expr;
mod raw_block;
mod render_directive;
mod rust_block;
mod rust_expr;
mod rust_expr_paren;
mod rust_expr_simple;
mod section_block;
mod section_directive;
mod template;
mod text;
mod use_directive;

use crate::config::Config;
use crate::error::rename_rules;
use crate::node::*;
use crate::parser::block::BlockParser;
use crate::parser::comment_block::CommentBlockParser;
use crate::parser::component_tag::ComponentTagParser;
use crate::parser::extends_directive::ExtendsDirectiveParser;
use crate::parser::include_directive::IncludeDirectiveParser;
use crate::parser::inner_text::InnerTextParser;
use crate::parser::match_expr::MatchExprParser;
use crate::parser::raw_block::RawBlockParser;
use crate::parser::render_directive::RenderDirectiveParser;
use crate::parser::rust_block::RustBlockParser;
use crate::parser::rust_expr::RustExprParser;
use crate::parser::rust_expr_paren::RustExprParenParser;
use crate::parser::rust_expr_simple::RustExprSimpleParser;
use crate::parser::section_block::SectionBlockParser;
use crate::parser::section_directive::SectionDirectiveParser;
use crate::parser::template::TemplateParser;
use crate::parser::text::TextParser;
use crate::parser::use_directive::UseDirectiveParser;
use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair, Pairs};
use pest::{Parser, Span};
use pest_derive::Parser;
use std::collections::HashSet;

#[derive(Parser)]
#[grammar = "rshtml.pest"]
pub struct RsHtmlParser {
    included_templates: HashSet<String>,
    config: Config,
    files: Vec<String>,
}

impl RsHtmlParser {
    pub fn new() -> Self {
        Self {
            included_templates: HashSet::new(),
            config: Config::default(),
            files: Vec::new(),
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
                Rule::extends_directive
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
            Rule::block => BlockParser::parse(self, pair),
            Rule::include_directive => IncludeDirectiveParser::parse(self, pair),
            Rule::render_directive => RenderDirectiveParser::parse(self, pair),
            Rule::render_body_directive => Ok(Node::RenderBody),
            Rule::extends_directive => ExtendsDirectiveParser::parse(self, pair),
            Rule::rust_block => RustBlockParser::parse(self, pair),
            Rule::rust_expr_simple => RustExprSimpleParser::parse(self, pair),
            Rule::rust_expr_paren => RustExprParenParser::parse(self, pair),
            Rule::rust_expr => RustExprParser::parse(self, pair),
            Rule::match_expr => MatchExprParser::parse(self, pair),
            Rule::section_directive => SectionDirectiveParser::parse(self, pair),
            Rule::section_block => SectionBlockParser::parse(self, pair),
            Rule::component_tag => ComponentTagParser::parse(self, pair),
            Rule::child_content_directive => Ok(Node::ChildContent),
            Rule::raw_block => RawBlockParser::parse(self, pair),
            Rule::use_directive => UseDirectiveParser::parse(self, pair),
            Rule::continue_directive => Ok(Node::ContinueDirective),
            Rule::break_directive => Ok(Node::BreakDirective),
            rule => Err(Box::new(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Error: Unknown rule: {rule:?}"),
                },
                pair.as_span(),
            ))),
        }
    }

    fn parse_template(&mut self, path: &str) -> Result<Node, Box<Error<Rule>>> {
        let input = self.read_template(path).map_err(|err| {
            Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Error reading template: {err:?}, path: {path}"),
                },
                Span::new(path, 0, 0).unwrap(),
            )
        })?;

        let mut pairs = Self::parse(Rule::template, &input)?;
        let template_pair = pairs.next().ok_or(Error::new_from_pos(
            ErrorVariant::CustomError {
                message: "Error: Empty template".to_string(),
            },
            pest::Position::new("Template", 0).unwrap(),
        ))?;

        if template_pair.as_rule() == Rule::template {
            self.files.push(path.to_string());

            let ast = self.build_ast_node(template_pair)?;

            self.files.pop();

            Ok(ast)
        } else {
            let err: Error<Rule> = Error::new_from_span(
                ErrorVariant::ParsingError {
                    positives: vec![Rule::template],
                    negatives: vec![],
                },
                template_pair.as_span(),
            );
            Err(Box::new(err))
        }
    }

    fn read_template(&self, path: &str) -> Result<String, String> {
        let view_path = self.config.views.0.join(path);
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
