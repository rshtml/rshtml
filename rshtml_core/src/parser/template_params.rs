use pest::{error::Error, iterators::Pair};

use crate::{
    error::E,
    node::Node,
    parser::{IParser, RsHtmlParser, Rule},
    position::Position,
};

pub struct TemplateParamsParser;

impl IParser for TemplateParamsParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        // let pair_span = pair.as_span();
        let position = Position::from(&pair);

        let param_pairs = pair
            .into_inner()
            // .find(|p| p.as_rule() == Rule::params)
            // .ok_or(E::pos(Rule::params).span(pair_span))
            // .into_iter()
            .filter(|p| p.as_rule() == Rule::param);

        let mut params = Vec::new();
        for param_pair in param_pairs {
            let param_pair_span = param_pair.as_span();
            let param_position = Position::from(&param_pair);

            let mut param_inner_pair = param_pair.into_inner();

            let param_name_pair = param_inner_pair
                .find(|p| p.as_rule() == Rule::param_name)
                .ok_or(E::pos(Rule::param_name).span(param_pair_span))?;

            let param_type = param_inner_pair
                .find(|p| p.as_rule() == Rule::param_type)
                .map(|p| match p.as_str() {
                    "Block" => "impl ::std::fmt::Display".to_string(),
                    other => other.to_string(),
                })
                .unwrap_or("impl ::std::fmt::Display".into());

            params.push((
                param_name_pair.as_str().to_string(),
                param_type,
                param_position,
            ));
        }

        Ok(Node::TemplateParams(params, position))
    }
}
