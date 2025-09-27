use crate::Node;
use crate::error::E;
use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::position::Position;
use pest::error::Error;
use pest::iterators::Pair;
use std::path::PathBuf;

pub struct ExtendsDirectiveParser;

impl IParser for ExtendsDirectiveParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();
        let position = Position::from(&pair);

        let mut path_str = parser.config.layout.clone();
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
                return Err(
                    E::mes(format!("Error parsing layout file '{path_str}': {err}"))
                        .span(pair_span),
                );
            }
        };

        let layout_node = match layout_node {
            Node::Template(file, nodes, _) => Node::Template(file, nodes, position),
            _ => {
                return Err(
                    E::mes("The layout file must contain Template as the top node.")
                        .span(pair_span),
                );
            }
        };

        Ok(Node::ExtendsDirective(
            PathBuf::from(path_str),
            Box::new(layout_node),
        ))
    }
}
