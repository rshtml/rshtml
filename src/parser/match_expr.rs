use crate::Node;
use crate::config::Config;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::iterators::Pair;
use std::collections::HashSet;

pub struct MatchExprParser;

impl IParser for MatchExprParser {
    fn parse(parser: &RsHtmlParser, pair: Pair<Rule>, config: &Config, included_templates: &HashSet<String>) -> Result<Node, String> {
        let mut pairs = pair.into_inner();

        let match_expr_head = pairs.find(|pair| pair.as_rule() == Rule::match_expr_head).unwrap();

        let match_expr_arms = pairs.filter(|pair| pair.as_rule() == Rule::match_expr_arm);

        let mut nodes: Vec<(String, Vec<Node>)> = Vec::new();
        for match_expr_arm in match_expr_arms {
            let mut match_expr_arm_iter = match_expr_arm.into_inner();

            let match_expr_arm_head = match_expr_arm_iter.next().unwrap();
            if match_expr_arm_head.as_rule() != Rule::match_expr_arm_head {
                return Err("Invalid match expression arm head.".to_string());
            }

            let match_expr_arm_value = match_expr_arm_iter.next().unwrap();
            let node_arm_value = match match_expr_arm_value.as_rule() {
                Rule::inner_template => parser.build_nodes_from_pairs(match_expr_arm_value.into_inner(), config, included_templates)?,
                Rule::rust_expr_simple => vec![parser.build_ast_node(match_expr_arm_value, config, included_templates)?],
                Rule::match_inner_text => vec![Node::InnerText(match_expr_arm_value.as_str().replace("@@", "@").replace("@@{", "{").replace("@@}", "}"))],
                _ => return Err("Unexpected match expression arm value ".to_string()),
            };

            nodes.push((match_expr_arm_head.as_str().to_string(), node_arm_value));
        }

        Ok(Node::MatchExpr(match_expr_head.as_str().to_string(), nodes))
    }
}
