use syn::{ExprMatch, parse_str};

use crate::{analyzer::Analyzer, diagnostic::Level, node::Node, position::Position};

pub struct MatchExprAnalyzer;

impl MatchExprAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        head: &String,
        arms: &Vec<(String, Vec<Node>)>,
        position: &Position,
    ) {
        let match_expr = head.to_owned() + "{}";
        if let Err(e) = parse_str::<ExprMatch>(&match_expr) {
            analyzer.diagnostic(
                position,
                "attempt to use invalid match expression",
                &[],
                &e.to_string(),
                head.len(),
                Level::Caution,
            );
        }

        let mut match_expr = head.to_owned() + "{";

        for (arm_name, arm_nodes) in arms {
            match_expr.push_str(&(arm_name.to_owned() + " => {},"));

            for node in arm_nodes {
                analyzer.analyze(node)
            }
        }
        // TODO: take match arm name position

        match_expr += "}";
        dbg!("{}", match_expr.to_string());

        if let Err(e) = parse_str::<ExprMatch>(&match_expr) {
            analyzer.diagnostic(
                position,
                "attempt to use invalid match expression",
                &[],
                &e.to_string(),
                head.len(),
                Level::Caution,
            );
        }
    }
}
