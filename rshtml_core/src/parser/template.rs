use crate::error::E;
use crate::node::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::position::Position;
use pest::error::Error;
use pest::iterators::Pair;
use std::path::PathBuf;

pub struct TemplateParser;

impl IParser for TemplateParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let span = pair.as_span();
        let position = Position::from(&pair);

        let file = parser
            .files
            .last()
            .ok_or_else(|| E::mes("No file found for component!").span(span))?;

        let component_path = PathBuf::from(file);
        let component_name = parser.extract_component_name(file).ok_or(
            E::mes(format!(
                "Failed to derive component name from import path: '{file:#?}'"
            ))
            .span(span),
        )?;

        parser.fn_names = Vec::new();
        let body = parser.build_nodes_from_pairs(pair.into_inner())?;
        let fn_names = parser.fn_names.to_owned();
        parser.fn_names = Vec::new();

        Ok(Node::Template(
            component_path,
            component_name,
            fn_names,
            body,
            position,
        ))
    }
}
