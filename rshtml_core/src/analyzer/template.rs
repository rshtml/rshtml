use crate::{analyzer::Analyzer, node::Node, position::Position};
use anyhow::Result;

pub struct TemplateAnalyzer;

impl TemplateAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        file: &String,
        nodes: &Vec<Node>,
        position: &Position,
    ) -> Result<()> {
        if !file.is_empty() {
            analyzer.files.push((file.clone(), position.clone()));
        }

        for node in nodes {
            analyzer.analyze(node)?;
        }

        if !file.is_empty() {
            analyzer.files.pop();
        }

        Ok(())
    }
}
