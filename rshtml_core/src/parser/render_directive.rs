use crate::Node;
use crate::error::E;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;

pub struct RenderDirectiveParser;

impl IParser for RenderDirectiveParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let span = pair.as_span();

        let path_pair = pair
            .into_inner()
            .find(|p| p.as_rule() == Rule::string_line)
            .ok_or(E::mes("No path found").span(span))?;

        let path_str = path_pair
            .as_str()
            .trim_matches('"')
            .trim_matches('\'')
            .to_string();

        Ok(Node::RenderDirective(path_str))
    }
}
