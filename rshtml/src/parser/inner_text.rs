use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;

pub struct InnerTextParser;

impl IParser for InnerTextParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        Ok(Node::InnerText(pair.as_str().replace("@@", "@").replace("@@{", "{").replace("@@}", "}")))
    }
}
