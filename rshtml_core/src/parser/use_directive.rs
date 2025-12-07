use crate::Node;
use crate::error::E;
use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::position::Position;
use pest::error::Error;
use pest::iterators::Pair;
use std::path::Path;

pub struct UseDirectiveParser;

impl IParser for UseDirectiveParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();
        let position = Position::from(&pair);

        let mut inner_pairs = pair.into_inner();
        let import_path_str = inner_pairs
            .find(|p| p.as_rule() == Rule::string_line)
            .ok_or(E::pos(Rule::string_line).span(pair_span))?;

        let mut import_path_str = import_path_str.as_str().trim_matches('"').to_string();
        if !import_path_str.ends_with(".rs.html") {
            import_path_str.push_str(".rs.html");
        }
        let import_path = Path::new(&import_path_str);

        let component_name = inner_pairs
            .find(|p| p.as_rule() == Rule::rust_identifier)
            .map(|p| p.as_str().to_string());

        let component_node = match parser.parse_template(&import_path_str) {
            Ok(node) => node,
            Err(err) => {
                return Err(E::mes(format!(
                    "Error parsing component file '{import_path_str}': {err}"
                ))
                .span(pair_span));
            }
        };

        let (component_node, component_name) = match component_node {
            Node::Template(file, name, nodes, _) => {
                let component_name = component_name.unwrap_or(name);

                (
                    Node::Template(file, component_name.to_owned(), nodes, position.to_owned()),
                    component_name,
                )
            }
            _ => {
                return Err(
                    E::mes("The component file must contain Template as the top node.")
                        .span(pair_span),
                );
            }
        };

        Ok(Node::UseDirective(
            component_name,
            import_path.to_path_buf(),
            Box::new(component_node),
            position,
        ))
    }
}
