use crate::node::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::position::Position;
use pest::error::Error;
use pest::iterators::Pair;

pub struct TemplateParser;

impl IParser for TemplateParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let position = Position::from(&pair);

        Ok(Node::Template(
            parser.files.last().cloned().unwrap_or_default(),
            parser.build_nodes_from_pairs(pair.into_inner())?,
            position,
        ))
    }
}
