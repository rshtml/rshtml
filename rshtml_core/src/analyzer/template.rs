use crate::{
    analyzer::{Analyzer, use_directive::UseDirectiveAnalyzer},
    node::Node,
    position::Position,
};

pub struct TemplateAnalyzer;

impl TemplateAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        file: &str,
        _name: &str,
        _fn_names: &Vec<String>,
        nodes: &Vec<Node>,
        position: &Position,
    ) {
        if !file.is_empty() {
            analyzer.files.push((file.to_owned(), position.clone()));
        }

        for node in nodes {
            analyzer.analyze(node)
        }

        analyzer
            .components
            .entry(analyzer.component.path.clone())
            .or_insert(analyzer.component.clone());
        UseDirectiveAnalyzer::analyze_uses(analyzer);

        if !file.is_empty() {
            analyzer.files.pop();
        }
    }
}
