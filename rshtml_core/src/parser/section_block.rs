use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::Node;
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub struct SectionBlockParser;

impl IParser for SectionBlockParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();

        let section_head_pair = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::section_head)
            .ok_or(Error::new_from_span(
                ErrorVariant::ParsingError {
                    positives: vec![Rule::section_head],
                    negatives: vec![],
                },
                pair_span,
            ))?;

        let section_name_pair = section_head_pair
            .into_inner()
            .find(|p| p.as_rule() == Rule::rust_identifier)
            .ok_or(Error::new_from_span(
                ErrorVariant::ParsingError {
                    positives: vec![Rule::rust_identifier],
                    negatives: vec![],
                },
                pair_span,
            ))?;

        let section_head = section_name_pair
            .as_str()
            .trim_matches('"')
            .trim_matches('\'')
            .to_string();

        let inner_pairs = pair
            .into_inner()
            .find(|x| x.as_rule() == Rule::inner_template)
            .ok_or(Error::new_from_span(
                ErrorVariant::ParsingError {
                    positives: vec![Rule::inner_template],
                    negatives: vec![],
                },
                pair_span,
            ))?;

        let body = parser.build_nodes_from_pairs(inner_pairs.into_inner())?;
        Ok(Node::SectionBlock(section_head, body))
    }
}
