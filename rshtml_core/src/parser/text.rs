use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;

pub struct TextParser;

impl IParser for TextParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let text = pair
            .as_str()
            .replace("@@", "@")
            .replace("@@{", "{")
            .replace("@@}", "}");
        Ok(Node::Text(text))
    }
}
