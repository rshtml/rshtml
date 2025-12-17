use crate::{analyzer::Analyzer, diagnostic::Level, node::Node, position::Position};
use syn::{Expr, parse_str};

pub struct RustExprAnalyzer;

impl RustExprAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer, exprs: &Vec<(String, Vec<Node>)>, position: &Position) {
        let mut rust_expr = String::new();

        for (expr, inner_nodes) in exprs {
            rust_expr += &(expr.to_owned() + "{}");

            for inner_node in inner_nodes {
                analyzer.analyze(inner_node)
            }
        }

        // TODO: take expr positions

        if let Err(e) = parse_str::<Expr>(&rust_expr) {
            analyzer.diagnostic(
                position,
                "attempt to use invalid statement",
                &[],
                &e.to_string(),
                1,
                Level::Caution,
            );
        }
    }
}
