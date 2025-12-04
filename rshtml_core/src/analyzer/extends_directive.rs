use crate::{analyzer::Analyzer, node::Node};
use std::path::PathBuf;

pub struct ExtendsDirectiveAnalyzer;

impl ExtendsDirectiveAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer, path: &PathBuf, layout: &Node) {
        analyzer.layout_directive = path.to_owned();
        analyzer.layout = Some(layout.to_owned());
    }
}
