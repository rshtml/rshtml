use crate::analyzer::Analyzer;

pub struct ChildContentAnalyzer;

impl ChildContentAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer) {
        if let Some(name) = &analyzer.is_component {
            analyzer
                .components
                .entry(name.clone())
                .and_modify(|c| c.has_child_content = true);
        }
    }
}
