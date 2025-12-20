use crate::Node;
use crate::error::E;
use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::position::Position;
use pest::error::Error;
use pest::iterators::Pair;

pub struct MatchExprParser;

impl IParser for MatchExprParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();
        let position = Position::from(&pair);

        let mut pairs = pair.into_inner();

        let match_expr_head = pairs
            .find(|pair| pair.as_rule() == Rule::match_expr_head)
            .ok_or(E::mes("Match expression head cannot find.").span(pair_span))?;

        let match_expr_arms = pairs.filter(|pair| pair.as_rule() == Rule::match_expr_arm);

        let mut nodes: Vec<(String, Position, Vec<Node>)> = Vec::new();
        for match_expr_arm in match_expr_arms {
            let match_expr_arm_span = match_expr_arm.as_span();
            let mut match_expr_arm_iter = match_expr_arm.into_inner();

            let match_expr_arm_head = match_expr_arm_iter.next().ok_or(
                E::mes("Match expression arm head cannot find.").span(match_expr_arm_span),
            )?;
            let match_expr_arm_head_position = Position::from(&match_expr_arm_head);

            if match_expr_arm_head.as_rule() != Rule::match_expr_arm_head {
                return Err(E::mes("Invalid match expression arm head.").span(match_expr_arm_span));
            }

            let match_expr_arm_value = match_expr_arm_iter.next().ok_or(
                E::mes("Match expression arm value cannot find.").span(match_expr_arm_span),
            )?;

            let node_arm_value = match match_expr_arm_value.as_rule() {
                Rule::inner_template => {
                    parser.build_nodes_from_pairs(match_expr_arm_value.into_inner())?
                }
                Rule::continue_directive => {
                    vec![Node::ContinueDirective]
                }
                Rule::break_directive => vec![Node::BreakDirective],
                Rule::rust_expr_paren => vec![parser.build_ast_node(match_expr_arm_value)?],
                Rule::rust_expr_simple => vec![parser.build_ast_node(match_expr_arm_value)?],
                Rule::match_inner_text => {
                    vec![Node::Text(match_expr_arm_value.as_str().replace("@@", "@"))]
                }
                _ => {
                    return Err(
                        E::mes("Unexpected match expression arm value ").span(match_expr_arm_span)
                    );
                }
            };

            nodes.push((
                match_expr_arm_head.as_str().to_string(),
                match_expr_arm_head_position,
                node_arm_value,
            ));
        }

        Ok(Node::MatchExpr(
            match_expr_head.as_str().to_string(),
            nodes,
            position,
        ))
    }
}
