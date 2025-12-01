use crate::{analyzer::Analyzer, node::Node, position::Position};

pub struct MatchExprAnalyzer;

impl MatchExprAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        _head: &String,
        arms: &Vec<(String, Vec<Node>)>,
        _position: &Position,
    ) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();
        for (_arm_name, arm_nodes) in arms {
            for node in arm_nodes {
                if let Err(e) = analyzer.analyze(node) {
                    errs.extend(e);
                }
            }
        }

        errs.is_empty().then(|| ()).ok_or(errs)
    }
}
