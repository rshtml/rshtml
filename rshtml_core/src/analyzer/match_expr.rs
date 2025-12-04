use crate::{analyzer::Analyzer, node::Node, position::Position};

pub struct MatchExprAnalyzer;

impl MatchExprAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        _head: &String,
        arms: &Vec<(String, Vec<Node>)>,
        _position: &Position,
    ) {
        for (_arm_name, arm_nodes) in arms {
            for node in arm_nodes {
                analyzer.analyze(node)
            }
        }
    }
}
