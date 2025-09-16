use crate::Node;
use crate::parser::Rule::rust_block_content;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub struct RustBlockParser;

impl IParser for RustBlockParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();
        Ok(Node::RustBlock(
            pair.into_inner()
                .find(|p| p.as_rule() == Rule::rust_block_content)
                .map(|p| p.as_str().to_string())
                .ok_or(Error::new_from_span(
                    ErrorVariant::ParsingError {
                        positives: vec![rust_block_content],
                        negatives: vec![],
                    },
                    pair_span,
                ))?,
        ))
    }
}
