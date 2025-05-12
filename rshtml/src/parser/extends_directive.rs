use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;
use std::path::PathBuf;

pub struct ExtendsDirectiveParser;

impl IParser for ExtendsDirectiveParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        let span = pair.as_span();

        let path_pair = pair.into_inner().find(|p| p.as_rule() == Rule::string_line).ok_or(Error::new_from_span(
            ErrorVariant::CustomError {
                message: "No path found".into(),
            },
            span,
        ))?;
        let path_str = path_pair.as_str().trim_matches('"').trim_matches('\'').to_string();

        // let layout = parser
        //     .read_template(&path_str)
        //     .or_else(|err| Err(Error::new_from_span(ErrorVariant::CustomError { message: err }, span)))?;
        let layout_node = parser.parse_template(&path_str)?;
        // TODO: manage error like include directive management

        Ok(Node::ExtendsDirective(PathBuf::from(path_str), Box::new(layout_node)))
    }
}
