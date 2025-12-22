use crate::error::E;
use crate::node::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::position::Position;
use pest::error::Error;
use pest::iterators::Pair;
use std::mem;
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

        let prev_fns = mem::take(&mut parser.fns);

        let body = parser.build_nodes_from_pairs(pair.into_inner())?;

        let fns = mem::replace(&mut parser.fns, prev_fns);

        Ok(Node::Template(
            component_path,
            component_name,
            fns,
            body,
            position,
        ))
    }
}
