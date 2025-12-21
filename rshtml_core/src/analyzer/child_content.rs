use crate::analyzer::Analyzer;

pub struct ChildContentAnalyzer;

impl ChildContentAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer) {
        analyzer.component.has_child_content = true;
    }
}
