use crate::{analyzer::Analyzer, node::Node, position::Position};
use anyhow::Result;

pub struct SectionBlockAnalyzer;

impl SectionBlockAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer, name: &String, content: &Vec<Node>) -> Result<()> {
        for node in content {
            analyzer.analyze(node)?;
        }

        analyzer
            .sections
            .insert(name.to_owned(), Position::default());
        // TODO: position must come from node, this is for now

        Ok(())
    }
}
