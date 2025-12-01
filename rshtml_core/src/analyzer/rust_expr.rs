use crate::{analyzer::Analyzer, node::Node, position::Position};

pub struct RustExprAnalyzer;

impl RustExprAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        exprs: &Vec<(String, Vec<Node>)>,
        _position: &Position,
    ) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();
        for (_expr, inner_nodes) in exprs {
            for inner_node in inner_nodes {
                if let Err(e) = analyzer.analyze(inner_node) {
                    errs.extend(e);
                }
            }
        }

        errs.is_empty().then(|| ()).ok_or(errs)
    }
}
