use crate::node::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;

pub struct RustExprParenParser;

impl IParser for RustExprParenParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        Ok(Node::RustExprParen(pair.as_str().to_string()))
    }
}
