use crate::Node;
use crate::config::Config;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::iterators::Pair;
use std::collections::HashSet;

pub struct ImportDirectiveParser;

impl IParser for ImportDirectiveParser {
    fn parse(_: &RsHtmlParser, pair: Pair<Rule>, _: &Config, _: &HashSet<String>) -> Result<Node, String> {
        let component_name = pair.clone().into_inner().find(|p| p.as_rule() == Rule::rust_identifier).unwrap().as_str().to_string();
        let import_path = pair.into_inner().find(|p| p.as_rule() == Rule::string_line).unwrap().as_str().to_string();

        Ok(Node::ImportDirective(component_name, import_path))
    }
}
