use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::position::Position;
use crate::Node;
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;
use std::path::PathBuf;

pub struct ExtendsDirectiveParser;

impl IParser for ExtendsDirectiveParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();
        let position = Position::from(&pair);

        let mut path_str = parser.config.views.1.clone();
        if let Some(path_pair) = pair.into_inner().find(|p| p.as_rule() == Rule::string_line) {
            path_str = path_pair
                .as_str()
                .trim_matches('"')
                .trim_matches('\'')
                .to_string();
        }

        let layout_node = match parser.parse_template(&path_str) {
            Ok(node) => node,
            Err(err) => {
                let include_template_error = Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("Error parsing layout file '{path_str}': {err}"),
                    },
                    pair_span,
                );

                return Err(Box::new(include_template_error));
            }
        };

        Ok(Node::ExtendsDirective(
            PathBuf::from(path_str),
            Box::new(layout_node),
            position,
        ))
    }
}
