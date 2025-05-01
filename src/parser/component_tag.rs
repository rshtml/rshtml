use crate::Node;
use crate::config::Config;
use crate::node::{ComponentParameter, ComponentParameterValue};
use crate::parser::component::ComponentParser;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::iterators::Pair;
use std::collections::HashSet;

pub struct ComponentTagParser;

impl IParser for ComponentTagParser {
    fn parse(parser: &RsHtmlParser, pair: Pair<Rule>, config: &Config, included_templates: &HashSet<String>) -> Result<Node, String> {
        let mut inner_pairs = pair.into_inner();
        let component_name = inner_pairs.find(|p| p.as_rule() == Rule::component_tag_name).unwrap().as_str().to_string();

        let component_parameter_pairs = inner_pairs.clone().filter(|p| p.as_rule() == Rule::attribute);

        let mut component_parameters = Vec::new();
        for pair in component_parameter_pairs {
            let pair_name = pair.clone().into_inner().find(|p| p.as_rule() == Rule::attribute_name).unwrap();

            let value = match pair.clone().into_inner().find(|p| p.as_rule() != Rule::attribute_name) {
                Some(pair_value) => ComponentParser::build_component_parameter_value(parser, pair_value, config, included_templates)?,
                None => ComponentParameterValue::Bool(true),
            };

            let name = pair_name.as_str().to_string();

            component_parameters.push(ComponentParameter { name, value });
        }

        let body = match inner_pairs.find(|x| x.as_rule() == Rule::tag_template) {
            Some(tag_template) => parser.build_nodes_from_pairs(tag_template.into_inner(), config, included_templates)?,
            None => vec![],
        };

        Ok(Node::Component(component_name, component_parameters, body))
    }
}
