use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;

pub struct InnerTextParser;

impl IParser for InnerTextParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        let inner_text = pair.as_str().replace("@@", "@").replace("@@{", "{").replace("@@}", "}");
        Ok(Node::InnerText(inner_text))
    }
}
