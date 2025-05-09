use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub struct MatchExprParser;

impl IParser for MatchExprParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        let pair_span = pair.as_span();
        let mut pairs = pair.into_inner();

        let match_expr_head = pairs.find(|pair| pair.as_rule() == Rule::match_expr_head).ok_or(Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Match expression head cannot find.".to_string(),
            },
            pair_span,
        ))?;

        let match_expr_arms = pairs.filter(|pair| pair.as_rule() == Rule::match_expr_arm);

        let mut nodes: Vec<(String, Vec<Node>)> = Vec::new();
        for match_expr_arm in match_expr_arms {
            let match_expr_arm_span = match_expr_arm.as_span();
            let mut match_expr_arm_iter = match_expr_arm.into_inner();

            let match_expr_arm_head = match_expr_arm_iter.next().ok_or(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: "Match expression arm head cannot find.".to_string(),
                },
                match_expr_arm_span,
            ))?;
            if match_expr_arm_head.as_rule() != Rule::match_expr_arm_head {
                return Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Invalid match expression arm head.".to_string(),
                    },
                    match_expr_arm_span,
                ));
            }

            let match_expr_arm_value = match_expr_arm_iter.next().ok_or(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: "Match expression arm value cannot find.".to_string(),
                },
                match_expr_arm_span,
            ))?;
            let node_arm_value = match match_expr_arm_value.as_rule() {
                Rule::inner_template => parser.build_nodes_from_pairs(match_expr_arm_value.into_inner())?,
                Rule::rust_expr_simple => vec![parser.build_ast_node(match_expr_arm_value)?],
                Rule::match_inner_text => vec![Node::InnerText(
                    match_expr_arm_value.as_str().replace("@@", "@").replace("@@{", "{").replace("@@}", "}"),
                )],
                _ => {
                    return Err(Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: "Unexpected match expression arm value ".to_string(),
                        },
                        match_expr_arm_span,
                    ));
                }
            };

            nodes.push((match_expr_arm_head.as_str().to_string(), node_arm_value));
        }

        Ok(Node::MatchExpr(match_expr_head.as_str().to_string(), nodes))
    }
}
