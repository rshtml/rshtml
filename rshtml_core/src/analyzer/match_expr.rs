use crate::{analyzer::Analyzer, node::Node, position::Position};
use anyhow::Result;

pub struct MatchExprAnalyzer;

impl MatchExprAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        _head: &String,
        arms: &Vec<(String, Vec<Node>)>,
        _position: &Position,
    ) -> Result<()> {
        for (_arm_name, arm_nodes) in arms {
            for node in arm_nodes {
                analyzer.analyze(node)?;
            }
        }

        Ok(())
    }
}
