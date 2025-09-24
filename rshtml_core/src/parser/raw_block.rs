use crate::parser::Rule::raw_content;
use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::Node;
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub struct RawBlockParser;

impl IParser for RawBlockParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();

        Ok(Node::Raw(
            pair.into_inner()
                .find(|p| p.as_rule() == Rule::raw_content)
                .map(|p| p.as_str().to_string())
                .ok_or(Error::new_from_span(
                    ErrorVariant::ParsingError {
                        positives: vec![raw_content],
                        negatives: vec![],
                    },
                    pair_span,
                ))?,
        ))
    }
}
