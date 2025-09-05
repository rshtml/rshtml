use crate::node::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::str_extensions::*;
use pest::error::Error;
use pest::iterators::Pair;

pub struct RustExprSimpleParser;

impl IParser for RustExprSimpleParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_str = pair.as_str();
        Ok(Node::RustExprSimple(
            pair_str.escaped_or_raw(),
            pair_str.is_escaped(),
        ))
    }
}
