use crate::node::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::traits::IsEscaped;
use pest::error::Error;
use pest::iterators::Pair;

pub struct RustExprSimpleParser;

impl IParser for RustExprSimpleParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        let pair_str = pair.as_str();
        Ok(Node::RustExprSimple(pair_str.escaped_or_raw(), pair_str.is_escaped()))
    }
}
