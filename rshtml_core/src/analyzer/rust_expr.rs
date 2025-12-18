use crate::{analyzer::Analyzer, diagnostic::Level, node::Node, position::Position};
use syn::{Expr, parse_str};

pub struct RustExprAnalyzer;

impl RustExprAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        exprs: &Vec<(String, Position, Vec<Node>)>,
        _position: &Position,
    ) {
        let mut rust_expr = String::new();

        for (expr, expr_position, inner_nodes) in exprs {
            rust_expr += &(expr.to_owned() + "{}");

            if let Err(e) = parse_str::<Expr>(&rust_expr) {
                analyzer.diagnostic(
                    expr_position,
                    "attempt to use invalid statement",
                    &[],
                    &e.to_string(),
                    1,
                    Level::Caution,
                );
            }

            for inner_node in inner_nodes {
                analyzer.analyze(inner_node)
            }
        }
    }
}
