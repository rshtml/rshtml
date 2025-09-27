use crate::Node;
use crate::error::E;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;
use std::path::Path;

pub struct UseDirectiveParser;

impl IParser for UseDirectiveParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();

        let mut inner_pairs = pair.into_inner();
        let import_path_str = inner_pairs
            .find(|p| p.as_rule() == Rule::string_line)
            .ok_or(E::pos(Rule::string_line).span(pair_span))?;

        let mut import_path_str = import_path_str.as_str().trim_matches('"').to_string();
        if !import_path_str.ends_with(".rs.html") {
            import_path_str.push_str(".rs.html");
        }
        let import_path = Path::new(&import_path_str);

        let component_name = match inner_pairs.find(|p| p.as_rule() == Rule::rust_identifier) {
            Some(component_name_pair) => component_name_pair.as_str().to_string(),
            None => import_path
                .file_stem()
                .and_then(|stem1| Path::new(stem1).file_stem())
                .and_then(|stem2| stem2.to_str())
                .map(|s| s.to_string())
                .ok_or(
                    E::mes(format!(
                        "Failed to derive component name from import path: '{import_path:#?}'"
                    ))
                    .span(pair_span),
                )?,
        };

        let component_node = match parser.parse_template(&import_path_str) {
            Ok(node) => node,
            Err(err) => {
                return Err(E::mes(format!(
                    "Error parsing component file '{import_path_str}': {err}"
                ))
                .span(pair_span));
            }
        };

        Ok(Node::UseDirective(
            component_name.clone(),
            import_path.to_path_buf(),
            Box::new(component_node),
        ))
    }
}
