use crate::Node;
use crate::config::Config;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::iterators::Pair;
use std::collections::HashSet;

pub struct SectionBlockParser;

impl IParser for SectionBlockParser {
    fn parse(parser: &RsHtmlParser, pair: Pair<Rule>, config: &Config, included_templates: &HashSet<String>) -> Result<Node, String> {
        let section_head_pair = pair.clone().into_inner().find(|p| p.as_rule() == Rule::section_head).unwrap();

        let section_head = section_head_pair.as_str().trim_matches('"').trim_matches('\'').to_string();

        let inner_pairs = pair.into_inner().find(|x| x.as_rule() == Rule::inner_template).unwrap();

        let body = parser.build_nodes_from_pairs(inner_pairs.into_inner(), config, included_templates)?;
        Ok(Node::SectionBlock(section_head, body))
    }
}
