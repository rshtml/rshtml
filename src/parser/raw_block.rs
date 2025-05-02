use crate::Node;
use crate::config::Config;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::iterators::Pair;
use std::collections::HashSet;

pub struct RawBlockParser;

impl IParser for RawBlockParser {
    fn parse(_: &RsHtmlParser, pair: Pair<Rule>, _: &Config, _: &HashSet<String>) -> Result<Node, String> {
        Ok(Node::Raw(
            pair.into_inner().find(|p| p.as_rule() == Rule::raw_content).map(|p| p.as_str().to_string()).ok_or("raw block error")?,
        ))
    }
}
