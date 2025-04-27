use crate::Node;
use crate::config::Config;
use crate::node::SectionDirectiveContent;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::iterators::Pair;
use std::collections::HashSet;

pub struct SectionDirectiveParser;

impl IParser for SectionDirectiveParser {
    fn parse(_: &RsHtmlParser, pair: Pair<Rule>, _: &Config, _: &HashSet<String>) -> Result<Node, String> {
        let mut pairs = pair.clone().into_inner().filter(|p| p.as_rule() == Rule::string_line || p.as_rule() == Rule::rust_expr_simple);

        match (pairs.next(), pairs.next()) {
            (Some(name), Some(value)) => {
                let value_pair = match value.as_rule() {
                    Rule::string_line => {
                        let value = value.as_str().trim_matches('"').trim_matches('\'').to_string();
                        SectionDirectiveContent::Text(value)
                    }
                    Rule::rust_expr_simple => {
                        let value = value.as_str().to_string();
                        SectionDirectiveContent::RustExprSimple(value)
                    }
                    _ => unreachable!(),
                };

                let name = name.as_str().trim_matches('"').trim_matches('\'').to_string();

                Ok(Node::SectionDirective(name, value_pair))
            }
            _ => Err("Error: section_directive".to_string()),
        }
    }
}
