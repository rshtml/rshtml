use crate::Node;
use crate::parser::text::TextParser;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;

pub struct InnerTextParser;

impl IParser for InnerTextParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        let inner_text = pair.as_str().replace("@@", "@").replace("@@{", "{").replace("@@}", "}");

        let inner_text = TextParser::remove_extra_newlines(&inner_text);

        Ok(Node::InnerText(inner_text))
    }
}
