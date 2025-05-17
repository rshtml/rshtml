use crate::node::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;

pub struct RustExprSimpleParser;

impl IParser for RustExprSimpleParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        Ok(Node::RustExprSimple(pair.as_str().to_string()))
    }
}
