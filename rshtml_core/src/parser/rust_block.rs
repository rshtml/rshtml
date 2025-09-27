use crate::Node;
use crate::error::E;
use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::position::Position;
use pest::error::Error;
use pest::iterators::Pair;

pub struct RustBlockParser;

impl IParser for RustBlockParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();
        let position = Position::from(&pair);

        Ok(Node::RustBlock(
            pair.into_inner()
                .find(|p| p.as_rule() == Rule::rust_block_content)
                .map(|p| p.as_str().to_string())
                .ok_or(E::pos(Rule::rust_block_content).span(pair_span))?,
            position,
        ))
    }
}
