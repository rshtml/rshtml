use crate::Node;
use crate::error::E;
use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::position::Position;
use pest::error::Error;
use pest::iterators::Pair;
use std::path::PathBuf;

pub struct IncludeDirectiveParser;

impl IParser for IncludeDirectiveParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();
        let position = Position::from(&pair);

        let path_pair = pair
            .into_inner()
            .find(|p| p.as_rule() == Rule::string_line)
            .ok_or(E::mes("Error: Expected a path to the included file").span(pair_span))?;

        let path = path_pair
            .as_str()
            .trim_matches('"')
            .trim_matches('\'')
            .to_string();

        let inner_template = match parser.parse_template(&path) {
            Ok(node) => node,
            Err(err) => {
                return Err(
                    E::mes(format!("Error parsing included file '{path}': {err}")).span(pair_span),
                );
            }
        };

        let (file, nodes) = match inner_template {
            Node::Template(file, nodes, _) => (file, nodes),
            _ => {
                return Err(E::mes(format!(
                            "Error: Expected a template in the included file '{path}', found {inner_template:?}"
                        )).span(path_pair.as_span()));
            }
        };

        Ok(Node::IncludeDirective(
            PathBuf::from(path),
            Box::new(Node::Template(file, nodes, position)),
        ))
    }
}
