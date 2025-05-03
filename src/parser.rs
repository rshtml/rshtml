mod comment_block;
mod component;
mod component_tag;
mod extends_directive;
mod include_directive;
mod match_expr;
mod raw_block;
mod render_directive;
mod rust_block;
mod rust_expr;
mod section_block;
mod section_directive;
mod use_directive;

use crate::config::Config;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use std::collections::HashSet;

use crate::node::*;
use crate::parser::comment_block::CommentBlockParser;
use crate::parser::component::ComponentParser;
use crate::parser::component_tag::ComponentTagParser;
use crate::parser::extends_directive::ExtendsDirectiveParser;
use crate::parser::include_directive::IncludeDirectiveParser;
use crate::parser::match_expr::MatchExprParser;
use crate::parser::raw_block::RawBlockParser;
use crate::parser::render_directive::RenderDirectiveParser;
use crate::parser::rust_block::RustBlockParser;
use crate::parser::rust_expr::RustExprParser;
use crate::parser::section_block::SectionBlockParser;
use crate::parser::section_directive::SectionDirectiveParser;
use crate::parser::use_directive::UseDirectiveParser;

#[derive(Parser)]
#[grammar = "rshtml.pest"]
pub struct RsHtmlParser {
    included_templates: HashSet<String>,
    config: Config,
}

impl RsHtmlParser {
    fn build_nodes_from_pairs(&mut self, pairs: Pairs<Rule>) -> Result<Vec<Node>, String> {
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

    fn build_ast_node(&mut self, pair: Pair<Rule>) -> Result<Node, String> {
        match pair.as_rule() {
            Rule::template => Ok(Node::Template(self.build_nodes_from_pairs(pair.into_inner())?)),
            Rule::text => Ok(Node::Text(pair.as_str().replace("@@", "@").replace("@@{", "{").replace("@@}", "}"))),
            Rule::inner_text => Ok(Node::InnerText(pair.as_str().replace("@@", "@").replace("@@{", "{").replace("@@}", "}"))),
            Rule::comment_block => CommentBlockParser::parse(self, pair),
            Rule::block => self.build_ast_node(pair.into_inner().next().ok_or_else(|| "Error: Empty block".to_string())?),
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
            rule => Err(format!("Error: Unexpected rule: {:?}", rule)),
        }
    }

    fn parse_template(&mut self, input: &str) -> Result<Node, String> {
        let mut pairs = Self::parse(Rule::template, input).unwrap();
        let template_pair = pairs.next().unwrap();
        if template_pair.as_rule() == Rule::template {
            let ast = self.build_ast_node(template_pair)?;
            Ok(ast)
        } else {
            panic!("Expected 'template', found {:?}", template_pair.as_rule());
        }
    }

    fn start_parser(&mut self, input: &str, config: Config, included_templates: HashSet<String>) -> Result<Node, String> {
        self.included_templates = included_templates;
        self.config = config;

        self.parse_template(input)
    }
}

pub fn run(input: &str, config: Config) -> Result<(Pairs<Rule>, Node), String> {
    let mut rshtml_parser = RsHtmlParser {
        config: config.clone(),
        included_templates: HashSet::new(),
    };
    let node = rshtml_parser.start_parser(input, config, HashSet::new())?;
    let pairs = RsHtmlParser::parse(Rule::template, input).unwrap();

    Ok((pairs.clone(), node))
}

pub trait IParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, String>;
}
