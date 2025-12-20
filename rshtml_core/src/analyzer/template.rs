use crate::{analyzer::Analyzer, node::Node, position::Position};

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

        if !file.is_empty() {
            analyzer.files.pop();
        }
    }
}
