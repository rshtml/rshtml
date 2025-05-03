use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::Node;
use pest::iterators::Pair;

pub struct CommentBlockParser;

impl IParser for CommentBlockParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, String> {
        Ok(Node::Comment(
            pair.into_inner()
                .find(|p| p.as_rule() == Rule::comment_content)
                .map(|p| p.as_str().to_string())
                .ok_or("comment block error: could not convert to string")?,
        ))
    }
}
