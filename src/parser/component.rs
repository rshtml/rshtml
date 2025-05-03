use crate::Node;
use crate::node::{ComponentParameter, ComponentParameterValue};
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub struct ComponentParser;

impl IParser for ComponentParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        let component_name = pair.clone().into_inner().find(|p| p.as_rule() == Rule::rust_identifier).unwrap().as_str().to_string();

        let component_parameter_pairs = pair.clone().into_inner().filter(|p| p.as_rule() == Rule::component_parameter);

        let mut component_parameters = Vec::new();
        for pair in component_parameter_pairs {
            let pair_name = pair.clone().into_inner().find(|p| p.as_rule() == Rule::rust_identifier).unwrap();
            let pair_value = pair.clone().into_inner().find(|p| p.as_rule() != Rule::rust_identifier).unwrap();

            let value = Self::build_component_parameter_value(parser, pair_value)?;
            let name = pair_name.as_str().to_string();

            component_parameters.push(ComponentParameter { name, value });
        }

        let content_pairs = pair.into_inner().find(|x| x.as_rule() == Rule::inner_template).unwrap();

        let body = parser.build_nodes_from_pairs(content_pairs.into_inner())?;
        Ok(Node::Component(component_name, component_parameters, body))
    }
}

impl ComponentParser {
    pub fn build_component_parameter_value(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<ComponentParameterValue, Error<Rule>> {
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
                let block_nodes = parser.build_nodes_from_pairs(pair.into_inner())?;
                Ok(ComponentParameterValue::Block(block_nodes))
            }
            rule => Err(Error::new_from_span(ErrorVariant::CustomError {message: format!("Unexpected rule: {:?}", rule)}, pair.as_span())),
        }
    }
}
