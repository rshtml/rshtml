use crate::node::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;

pub struct TemplateParser;

impl IParser for TemplateParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        Ok(Node::Template(parser.build_nodes_from_pairs(pair.into_inner())?))
    }
}
