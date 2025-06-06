use crate::Node;
use crate::node::{ComponentParameter, ComponentParameterValue};
use crate::parser::component::ComponentParser;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub struct ComponentTagParser;

impl IParser for ComponentTagParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();
        let mut inner_pairs = pair.into_inner();
        let component_name_pair = inner_pairs.find(|p| p.as_rule() == Rule::component_tag_name).ok_or(Error::new_from_span(
            ErrorVariant::ParsingError {
                positives: vec![Rule::component_tag_name],
                negatives: vec![],
            },
            pair_span,
        ))?;
        let component_name = component_name_pair.as_str().to_string();

        let component_parameter_pairs = inner_pairs.clone().filter(|p| p.as_rule() == Rule::attribute);

        let mut component_parameters = Vec::new();
        for pair in component_parameter_pairs {
            let pair_name = pair
                .clone()
                .into_inner()
                .find(|p| p.as_rule() == Rule::attribute_name)
                .ok_or(Error::new_from_span(
                    ErrorVariant::ParsingError {
                        positives: vec![Rule::attribute_name],
                        negatives: vec![],
                    },
                    pair_span,
                ))?;

            let value = match pair.clone().into_inner().find(|p| p.as_rule() != Rule::attribute_name) {
                Some(pair_value) => ComponentParser::build_component_parameter_value(parser, pair_value)?,
                None => ComponentParameterValue::Bool(true),
            };

            let name = pair_name.as_str().to_string();

            component_parameters.push(ComponentParameter { name, value });
        }

        let body = match inner_pairs.find(|x| x.as_rule() == Rule::tag_template) {
            Some(tag_template) => parser.build_nodes_from_pairs(tag_template.into_inner())?,
            None => vec![],
        };

        Ok(Node::Component(component_name, component_parameters, body))
    }
}
