use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub struct SectionBlockParser;

impl IParser for SectionBlockParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        let section_head_pair = pair.clone().into_inner().find(|p| p.as_rule() == Rule::section_head).ok_or(Error::new_from_span(
            ErrorVariant::ParsingError {
                positives: vec![Rule::section_head],
                negatives: vec![],
            },
            pair.as_span(),
        ))?;

        let section_head = section_head_pair.as_str().trim_matches('"').trim_matches('\'').to_string();

        let inner_pairs = pair.into_inner().find(|x| x.as_rule() == Rule::inner_template).unwrap();

        let body = parser.build_nodes_from_pairs(inner_pairs.into_inner())?;
        Ok(Node::SectionBlock(section_head, body))
    }
}
