use crate::{analyzer::Analyzer, node::Node, position::Position};

pub struct RustExprAnalyzer;

impl RustExprAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        exprs: &Vec<(String, Vec<Node>)>,
        _position: &Position,
    ) {
        for (_expr, inner_nodes) in exprs {
            for inner_node in inner_nodes {
                analyzer.analyze(inner_node)
            }
        }
    }
}
