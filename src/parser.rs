mod component;
mod extends_directive;
mod import_directive;
mod include_directive;
mod match_expr;
mod render_directive;
mod rust_block;
mod rust_expr;
mod section_block;
mod section_directive;

use crate::config::Config;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use std::collections::HashSet;

use crate::node::*;
use crate::parser::component::ComponentParser;
use crate::parser::extends_directive::ExtendsDirectiveParser;
use crate::parser::import_directive::ImportDirectiveParser;
use crate::parser::include_directive::IncludeDirectiveParser;
use crate::parser::match_expr::MatchExprParser;
use crate::parser::render_directive::RenderDirectiveParser;
use crate::parser::rust_block::RustBlockParser;
use crate::parser::rust_expr::RustExprParser;
use crate::parser::section_block::SectionBlockParser;
use crate::parser::section_directive::SectionDirectiveParser;

#[derive(Parser)]
#[grammar = "rshtml.pest"]
pub struct RsHtmlParser;

impl RsHtmlParser {
    fn build_nodes_from_pairs(&self, pairs: Pairs<Rule>, config: &Config, included_templates: &HashSet<String>) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::extends_directive | Rule::comment_block | Rule::block | Rule::text | Rule::inner_text => {
                    nodes.push(self.build_ast_node(pair, config, included_templates)?);
                }
                // skip other rules (EOI, WHITESPACE, etc.)
                _ => {}
            }
        }
        Ok(nodes)
    }

    fn build_ast_node(&self, pair: Pair<Rule>, config: &Config, included_templates: &HashSet<String>) -> Result<Node, String> {
        match pair.as_rule() {
            Rule::template => Ok(Node::Template(self.build_nodes_from_pairs(pair.into_inner(), config, included_templates)?)),
            Rule::text => Ok(Node::Text(pair.as_str().replace("@@", "@").replace("@@{", "{").replace("@@}", "}"))),
            Rule::inner_text => Ok(Node::InnerText(pair.as_str().replace("@@", "@").replace("@@{", "{").replace("@@}", "}"))),
            Rule::comment_block => Ok(Node::Comment(
                pair.into_inner().find(|p| p.as_rule() == Rule::comment_content).map(|p| p.as_str().to_string()).unwrap_or_default(),
            )),
            Rule::block => self.build_ast_node(pair.into_inner().next().ok_or_else(|| "Error: Empty block".to_string())?, config, included_templates),
            Rule::include_directive => IncludeDirectiveParser::parse(self, pair, config, included_templates),
            Rule::render_directive => RenderDirectiveParser::parse(self, pair, config, included_templates),
            Rule::render_body_directive => Ok(Node::RenderBody),
            Rule::extends_directive => ExtendsDirectiveParser::parse(self, pair, config, included_templates),
            Rule::rust_block => RustBlockParser::parse(self, pair, config, included_templates),
            Rule::rust_expr_simple => Ok(Node::RustExprSimple(pair.as_str().to_string())),
            Rule::rust_expr_paren => Ok(Node::RustExprParen(pair.as_str().to_string())),
            Rule::rust_expr => RustExprParser::parse(self, pair, config, included_templates),
            Rule::match_expr => MatchExprParser::parse(self, pair, config, included_templates),
            Rule::section_directive => SectionDirectiveParser::parse(self, pair, config, included_templates),
            Rule::section_block => SectionBlockParser::parse(self, pair, config, included_templates),
            Rule::component => ComponentParser::parse(self, pair, config, included_templates),
            Rule::child_content_directive => Ok(Node::ChildContent),
            Rule::raw_block => Ok(Node::Raw(
                pair.into_inner().find(|p| p.as_rule() == Rule::raw_content).map(|p| p.as_str().to_string()).unwrap_or_default(),
            )),
            Rule::import_directive => ImportDirectiveParser::parse(self, pair, config, included_templates),
            rule => Err(format!("Error: Unexpected rule: {:?}", rule)),
        }
    }

    fn parse_template(&self, input: &str, config: &Config, included_templates: &HashSet<String>) -> Result<Node, String> {
        let mut pairs = Self::parse(Rule::template, input).unwrap();
        let template_pair = pairs.next().unwrap();
        if template_pair.as_rule() == Rule::template {
            let ast = self.build_ast_node(template_pair, config, included_templates)?;
            Ok(ast)
        } else {
            panic!("Expected 'template', found {:?}", template_pair.as_rule());
        }
    }
}

pub fn run<'a>(input: &'a str, config: &Config) -> Result<(Pairs<'a, Rule>, Node), String> {
    let rshtml_parser = RsHtmlParser {};
    let node = rshtml_parser.parse_template(input, config, &HashSet::new())?;
    let pairs = RsHtmlParser::parse(Rule::template, input).unwrap();

    Ok((pairs.clone(), node))
}

pub trait IParser {
    fn parse(parser: &RsHtmlParser, pair: Pair<Rule>, config: &Config, included_templates: &HashSet<String>) -> Result<Node, String>;
}
