use crate::{
    error::E,
    node::{Function, Node},
    parser::{IParser, RsHtmlParser, Rule, template_params::TemplateParamsParser},
    position::Position,
};
use pest::{error::Error, iterators::Pair};

pub struct FnDirectiveParser;

impl IParser for FnDirectiveParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();
        let position = Position::from(&pair);

        let mut inner_pairs = pair.into_inner();

        let fn_head_pair = inner_pairs
            .find(|p| p.as_rule() == Rule::fn_head)
            .ok_or(E::pos(Rule::fn_head).span(pair_span))?;
        let fn_head_pair_span = fn_head_pair.as_span();

        let fn_name = fn_head_pair
            .into_inner()
            .find(|p| p.as_rule() == Rule::rust_identifier)
            .map(|p| p.as_str().to_string())
            .ok_or(E::pos(Rule::fn_head).span(fn_head_pair_span))?;

        let params = inner_pairs
            .find(|p| p.as_rule() == Rule::fn_params)
            .map(|p| TemplateParamsParser::parse(parser, p))
            .transpose()?
            .map(|node| match node {
                Node::TemplateParams(params, _) => Ok(params),
                _ => Err(E::mes("Error parsing function parameters".to_string()).span(pair_span)),
            })
            .transpose()?
            .unwrap_or_default();

        let body = match inner_pairs.find(|p| p.as_rule() == Rule::inner_template) {
            Some(inner_template) => parser.build_nodes_from_pairs(inner_template.into_inner())?,
            _ => vec![],
        };

        parser.fns.push(Function {
            name: fn_name.to_owned(),
            params: params.to_owned(),
        });

        Ok(Node::FnDirective(fn_name, params, body, position))
    }
}
