use crate::Node;
use crate::error::E;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;

pub struct RawBlockParser;

impl IParser for RawBlockParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();

        Ok(Node::Raw(
            pair.into_inner()
                .find(|p| p.as_rule() == Rule::raw_content)
                .map(|p| p.as_str().to_string())
                .ok_or(E::pos(Rule::raw_content).span(pair_span))?,
        ))
    }
}
