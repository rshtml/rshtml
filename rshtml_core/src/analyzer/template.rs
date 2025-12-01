use crate::{analyzer::Analyzer, node::Node, position::Position};

pub struct TemplateAnalyzer;

impl TemplateAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        file: &str,
        nodes: &Vec<Node>,
        position: &Position,
    ) -> Result<(), Vec<String>> {
        if !file.is_empty() {
            analyzer.files.push((file.to_owned(), position.clone()));
        }

        let mut errs = Vec::new();
        for node in nodes {
            if let Err(e) = analyzer.analyze(node) {
                errs.extend(e);
            }
        }

        if !file.is_empty() {
            analyzer.files.pop();
        }

        errs.is_empty().then(|| ()).ok_or(errs)
    }
}
