use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub struct IncludeDirectiveParser;

impl IParser for IncludeDirectiveParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        let pair_span = pair.as_span();

        let path_pair = pair.into_inner().find(|p| p.as_rule() == Rule::string_line).ok_or(Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Error: Expected a path to the included file".to_string(),
            },
            pair_span,
        ))?;

        let path = path_pair.as_str().trim_matches('"').trim_matches('\'').to_string();

        let view_path = parser.config.views_base_path.join(&path);

        let canonical_path = view_path.canonicalize().unwrap_or_default().to_string_lossy().to_string();

        if parser.included_templates.contains(&canonical_path) {
            return Err(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Error: Circular include detected for file '{}'", path),
                },
                path_pair.as_span(),
            ));
        }

        parser.included_templates.insert(canonical_path.clone());

        let inner_template = match parser.parse_template(&path) {
            Ok(node) => node,
            Err(err) => {
                let include_template_error = Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("Error parsing included file '{}': {}", path, err),
                    },
                    pair_span,
                );

                return Err(include_template_error);
            }
        };

        parser.included_templates.remove(&canonical_path);

        let nodes = match inner_template {
            Node::Template(nodes) => nodes,
            _ => {
                return Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("Error: Expected a template in the included file '{}', found {:?}", path, inner_template),
                    },
                    path_pair.as_span(),
                ));
            }
        };

        Ok(Node::Template(nodes))
    }
}
