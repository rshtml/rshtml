use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::iterators::Pair;

pub struct RenderDirectiveParser;

impl IParser for RenderDirectiveParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, String> {
        let path_pair = pair.into_inner().find(|p| p.as_rule() == Rule::string_line).unwrap();
        let path_str = path_pair.as_str().trim_matches('"').trim_matches('\'').to_string();

        Ok(Node::RenderDirective(path_str))
    }
}
