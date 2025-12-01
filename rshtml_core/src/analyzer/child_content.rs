use crate::analyzer::Analyzer;

pub struct ChildContentAnalyzer;

impl ChildContentAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer) -> Result<(), Vec<String>> {
        if let Some(name) = &analyzer.is_component {
            analyzer
                .components
                .entry(name.clone())
                .and_modify(|component| component.has_child_content = true);
        }

        Ok(())
    }
}
