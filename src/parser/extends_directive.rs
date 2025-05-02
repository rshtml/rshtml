use crate::Node;
use crate::config::Config;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::iterators::Pair;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct ExtendsDirectiveParser;

impl IParser for ExtendsDirectiveParser {
    fn parse(_: &RsHtmlParser, pair: Pair<Rule>, _: &Config, _: &HashSet<String>) -> Result<Node, String> {
        let path_pair = pair.into_inner().find(|p| p.as_rule() == Rule::string_line).unwrap();
        let path_str = path_pair.as_str().trim_matches('"').trim_matches('\'').to_string();

        Ok(Node::ExtendsDirective(PathBuf::from(path_str)))
    }
}
