use crate::{analyzer::Analyzer, node::Node, position::Position};
use anyhow::Result;

pub struct RustExprAnalyzer;

impl RustExprAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        exprs: &Vec<(String, Vec<Node>)>,
        _position: &Position,
    ) -> Result<()> {
        for (_expr, inner_nodes) in exprs {
            for inner_node in inner_nodes {
                analyzer.analyze(inner_node)?;
            }
        }

        Ok(())
    }
}
