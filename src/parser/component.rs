use crate::Node;
use crate::config::Config;
use crate::node::{ComponentParameter, ComponentParameterValue};
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::iterators::Pair;
use std::collections::HashSet;

pub struct ComponentParser;

impl IParser for ComponentParser {
    fn parse(parser: &RsHtmlParser, pair: Pair<Rule>, config: &Config, included_templates: &HashSet<String>) -> Result<Node, String> {
        let component_name = pair.clone().into_inner().find(|p| p.as_rule() == Rule::rust_identifier).unwrap().as_str().to_string();

        let component_parameter_pairs = pair.clone().into_inner().filter(|p| p.as_rule() == Rule::component_parameter);

        let mut component_parameters = Vec::new();
        for pair in component_parameter_pairs {
            let pair_name = pair.clone().into_inner().find(|p| p.as_rule() == Rule::rust_identifier).unwrap();
            let pair_value = pair.clone().into_inner().find(|p| p.as_rule() != Rule::rust_identifier).unwrap();

            let value = Self::build_component_parameter_value(parser, pair_value, config, included_templates)?;
            let name = pair_name.as_str().to_string();

            component_parameters.push(ComponentParameter { name, value });
        }

        let content_pairs = pair.into_inner().find(|x| x.as_rule() == Rule::inner_template).unwrap();

        let body = parser.build_nodes_from_pairs(content_pairs.into_inner(), config, included_templates)?;
        Ok(Node::Component(component_name, component_parameters, body))
    }
}

impl ComponentParser {
    pub fn build_component_parameter_value(parser: &RsHtmlParser, pair: Pair<Rule>, config: &Config, included_templates: &HashSet<String>) -> Result<ComponentParameterValue, String> {
        match pair.as_rule() {
            Rule::bool => Ok(ComponentParameterValue::Bool(pair.as_str() == "true")),
            Rule::number => Ok(ComponentParameterValue::Number(pair.as_str().to_string())),
            Rule::string => {
                let raw_str = pair.as_str().trim_matches('"').trim_matches('\'');
                Ok(ComponentParameterValue::String(raw_str.to_string()))
            }
            Rule::rust_expr_simple => Ok(ComponentParameterValue::RustExprSimple(pair.as_str().to_string())),
            Rule::rust_expr_paren => Ok(ComponentParameterValue::RustExprParen(pair.as_str().to_string())),
            Rule::inner_template => {
                let block_nodes = parser.build_nodes_from_pairs(pair.into_inner(), config, included_templates)?;
                Ok(ComponentParameterValue::Block(block_nodes))
            }
            rule => Err(format!("Unexpected rule for component parameter value: {:?}", rule)),
        }
    }
}
