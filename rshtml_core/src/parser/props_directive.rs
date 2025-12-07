use pest::{error::Error, iterators::Pair};

use crate::{
    error::E,
    node::Node,
    parser::{IParser, RsHtmlParser, Rule},
    position::Position,
};

pub struct PropsDirectiveParser;

impl IParser for PropsDirectiveParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let position = Position::from(&pair);

        let prop_pairs = pair.into_inner().filter(|p| p.as_rule() == Rule::prop);

        let mut props = Vec::new();
        for prop_pair in prop_pairs {
            let prop_pair_span = prop_pair.as_span();
            let prop_position = Position::from(&prop_pair);

            let mut prop_inner_pair = prop_pair.into_inner();

            let prop_name_pair = prop_inner_pair
                .find(|p| p.as_rule() == Rule::prop_name)
                .ok_or(E::pos(Rule::prop_name).span(prop_pair_span))?;

            let prop_type_pair = prop_inner_pair
                .find(|p| p.as_rule() == Rule::prop_type)
                .map(|p| p.as_str().to_string())
                .unwrap_or("impl ::std::fmt::Display".into());

            props.push((
                prop_name_pair.as_str().to_string(),
                prop_type_pair,
                prop_position,
            ));
        }

        Ok(Node::PropsDirective(props, position))
    }
}
