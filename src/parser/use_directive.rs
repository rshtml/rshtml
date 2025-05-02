use crate::Node;
use crate::config::Config;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::iterators::Pair;
use std::collections::HashSet;

pub struct UseDirectiveParser;

impl IParser for UseDirectiveParser {
    fn parse(_: &RsHtmlParser, pair: Pair<Rule>, _: &Config, _: &HashSet<String>) -> Result<Node, String> {
        let mut inner_pairs = pair.into_inner();
        let import_path = inner_pairs.find(|p| p.as_rule() == Rule::string_line).unwrap().as_str().to_string();
        let component_name = inner_pairs.find(|p| p.as_rule() == Rule::rust_identifier).unwrap().as_str().to_string();

        Ok(Node::UseDirective(component_name, import_path))
    }
}
