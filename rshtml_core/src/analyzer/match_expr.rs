use crate::{analyzer::Analyzer, diagnostic::Level, node::Node, position::Position};
use syn::{ExprMatch, parse_str};

pub struct MatchExprAnalyzer;

impl MatchExprAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        head: &String,
        arms: &Vec<(String, Position, Vec<Node>)>,
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

        let mut lines = Vec::new();
        let mut err = String::new();

        for (arm_name, arm_name_position, arm_nodes) in arms {
            match_expr.push_str(&(arm_name.to_owned() + " => {},"));

            if let Err(e) = parse_str::<ExprMatch>(&[&match_expr, "}"].concat()) {
                lines.push(arm_name_position.0.0);
                if err.is_empty() {
                    err = e.to_string();
                }
            }

            for node in arm_nodes {
                analyzer.analyze(node)
            }
        }

        if !lines.is_empty() {
            analyzer.diagnostic(
                position,
                "attempt to use invalid match expression",
                &lines,
                &err,
                0,
                Level::Caution,
            );
        }
    }
}
