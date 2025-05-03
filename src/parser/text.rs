use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;

pub struct TextParser;

impl IParser for TextParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        Ok(Node::Text(pair.as_str().replace("@@", "@").replace("@@{", "{").replace("@@}", "}")))
    }
}
