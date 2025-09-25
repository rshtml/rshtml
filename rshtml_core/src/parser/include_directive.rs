use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::position::Position;
use crate::Node;
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub struct IncludeDirectiveParser;

impl IParser for IncludeDirectiveParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();
        let position = Position::from(&pair);

        let path_pair = pair
            .into_inner()
            .find(|p| p.as_rule() == Rule::string_line)
            .ok_or(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: "Error: Expected a path to the included file".to_string(),
                },
                pair_span,
            ))?;

        let path = path_pair
            .as_str()
            .trim_matches('"')
            .trim_matches('\'')
            .to_string();

        let inner_template = match parser.parse_template(&path) {
            Ok(node) => node,
            Err(err) => {
                let include_template_error = Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("Error parsing included file '{path}': {err}"),
                    },
                    pair_span,
                );

                return Err(Box::new(include_template_error));
            }
        };

        let (file, nodes) = match inner_template {
            Node::Template(file, nodes, _) => (file, nodes),
            _ => {
                return Err(Box::new(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!(
                            "Error: Expected a template in the included file '{path}', found {inner_template:?}"
                        ),
                    },
                    path_pair.as_span(),
                )));
            }
        };

        Ok(Node::Template(file, nodes, position))
    }
}
