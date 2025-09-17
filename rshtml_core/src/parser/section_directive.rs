use crate::node::Position;
use crate::node::SectionDirectiveContent;
use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::str_extensions::*;
use crate::Node;
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub struct SectionDirectiveParser;

impl IParser for SectionDirectiveParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();
        let position = Position::from(&pair);

        let mut pairs = pair
            .clone()
            .into_inner()
            .filter(|p| p.as_rule() == Rule::string_line || p.as_rule() == Rule::rust_expr_simple);

        match (pairs.next(), pairs.next()) {
            (Some(name), Some(value)) => {
                let value_pair = match value.as_rule() {
                    Rule::string_line => {
                        let value = value
                            .as_str()
                            .trim_matches('"')
                            .trim_matches('\'')
                            .to_string();
                        SectionDirectiveContent::Text(value)
                    }
                    Rule::rust_expr_simple => {
                        let value = value.as_str();
                        SectionDirectiveContent::RustExprSimple(
                            value.escaped_or_raw(),
                            value.is_escaped(),
                        )
                    }
                    _ => unreachable!(),
                };

                let name = name
                    .as_str()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();

                Ok(Node::SectionDirective(name, value_pair, position))
            }
            _ => Err(Box::new(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: "Error: section_directive".to_string(),
                },
                pair_span,
            ))),
        }
    }
}
