use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub struct BlockParser;

impl IParser for BlockParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();

        parser.build_ast_node(pair.into_inner().next().ok_or(Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Error: Empty block".to_string(),
            },
            pair_span,
        ))?)
    }
}
