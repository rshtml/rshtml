use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::position::Position;
use crate::Node;
use pest::error::Error;
use pest::iterators::Pair;

pub struct TextParser;

impl IParser for TextParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let position = Position::from(&pair);

        let text = pair
            .as_str()
            .replace("@@", "@")
            .replace("@@{", "{")
            .replace("@@}", "}");
        Ok(Node::Text(text, position))
    }
}
