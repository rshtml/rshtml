use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::Node;
use pest::iterators::Pair;

pub struct RawBlockParser;

impl IParser for RawBlockParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, String> {
        Ok(Node::Raw(
            pair.into_inner().find(|p| p.as_rule() == Rule::raw_content).map(|p| p.as_str().to_string()).ok_or("raw block error")?,
        ))
    }
}
