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
mod section_block;
mod section_directive;
mod text;
mod use_directive;

use crate::config::Config;
use crate::node::*;
use crate::parser::block::BlockParser;
use crate::parser::comment_block::CommentBlockParser;
use crate::parser::component::ComponentParser;
use crate::parser::component_tag::ComponentTagParser;
use crate::parser::extends_directive::ExtendsDirectiveParser;
use crate::parser::include_directive::IncludeDirectiveParser;
use crate::parser::inner_text::InnerTextParser;
use crate::parser::match_expr::MatchExprParser;
use crate::parser::raw_block::RawBlockParser;
use crate::parser::render_directive::RenderDirectiveParser;
use crate::parser::rust_block::RustBlockParser;
use crate::parser::rust_expr::RustExprParser;
use crate::parser::section_block::SectionBlockParser;
use crate::parser::section_directive::SectionDirectiveParser;
use crate::parser::text::TextParser;
use crate::parser::use_directive::UseDirectiveParser;
use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair, Pairs};
use pest::{Parser, Position};
use pest_derive::Parser;
use std::collections::HashSet;

#[derive(Parser)]
#[grammar = "rshtml.pest"]
pub struct RsHtmlParser {
    included_templates: HashSet<String>,
    config: Config,
}

impl RsHtmlParser {
    fn build_nodes_from_pairs(&mut self, pairs: Pairs<Rule>) -> Result<Vec<Node>, Error<Rule>> {
        let mut nodes = Vec::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::extends_directive | Rule::comment_block | Rule::block | Rule::text | Rule::inner_text => {
                    nodes.push(self.build_ast_node(pair)?);
                }
                // skip other rules (EOI, WHITESPACE, etc.)
                _ => {}
            }
        }
        Ok(nodes)
    }

    fn build_ast_node(&mut self, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        match pair.as_rule() {
            Rule::template => Ok(Node::Template(self.build_nodes_from_pairs(pair.into_inner())?)),
            Rule::text => TextParser::parse(self, pair),
            Rule::inner_text => InnerTextParser::parse(self, pair),
            Rule::comment_block => CommentBlockParser::parse(self, pair),
            Rule::block => BlockParser::parse(self, pair),
            Rule::include_directive => IncludeDirectiveParser::parse(self, pair),
            Rule::render_directive => RenderDirectiveParser::parse(self, pair),
            Rule::render_body_directive => Ok(Node::RenderBody),
            Rule::extends_directive => ExtendsDirectiveParser::parse(self, pair),
            Rule::rust_block => RustBlockParser::parse(self, pair),
            Rule::rust_expr_simple => Ok(Node::RustExprSimple(pair.as_str().to_string())),
            Rule::rust_expr_paren => Ok(Node::RustExprParen(pair.as_str().to_string())),
            Rule::rust_expr => RustExprParser::parse(self, pair),
            Rule::match_expr => MatchExprParser::parse(self, pair),
            Rule::section_directive => SectionDirectiveParser::parse(self, pair),
            Rule::section_block => SectionBlockParser::parse(self, pair),
            Rule::component => ComponentParser::parse(self, pair),
            Rule::component_tag => ComponentTagParser::parse(self, pair),
            Rule::child_content_directive => Ok(Node::ChildContent),
            Rule::raw_block => RawBlockParser::parse(self, pair),
            Rule::use_directive => UseDirectiveParser::parse(self, pair),
            rule => Err(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Error: Unknown rule: {:?}", rule),
                },
                pair.as_span(),
            )),
        }
    }

    fn parse_template(&mut self, input: &str) -> Result<Node, Error<Rule>> {
        let mut pairs = Self::parse(Rule::template, input)?;
        let template_pair = pairs.next().ok_or(Error::new_from_pos(
            ErrorVariant::CustomError {
                message: "Error: Empty template".to_string(),
            },
            Position::new("Template", 0).unwrap(),
        ))?;

        if template_pair.as_rule() == Rule::template {
            let ast = self.build_ast_node(template_pair)?;
            Ok(ast)
        } else {
            let err: Error<Rule> = Error::new_from_span(
                ErrorVariant::ParsingError {
                    positives: vec![Rule::template],
                    negatives: vec![],
                },
                template_pair.as_span(),
            );
            Err(err)
        }
    }

    fn start_parser(&mut self, input: &str, config: Config, included_templates: HashSet<String>) -> Result<Node, Error<Rule>> {
        self.included_templates = included_templates;
        self.config = config;

        self.parse_template(input)
    }
}

pub fn start_parser(input: &str, config: Config) -> Result<Node, Error<Rule>> {
    let mut rshtml_parser = RsHtmlParser {
        config,
        included_templates: HashSet::new(),
    };

    rshtml_parser.parse_template(input)
}

pub fn run(input: &str, config: Config) -> Result<(Pairs<Rule>, Node), Error<Rule>> {
    let mut rshtml_parser = RsHtmlParser {
        config: config.clone(),
        included_templates: HashSet::new(),
    };

    let node = rshtml_parser.start_parser(input, config, HashSet::new())?;
    let pairs = RsHtmlParser::parse(Rule::template, input)?;

    Ok((pairs.clone(), node))
}

pub trait IParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>>;
}
