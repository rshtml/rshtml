use crate::{
    analyzer::Analyzer,
    node::{Node, SectionDirectiveContent},
    position::Position,
};
use anyhow::Result;

pub struct SectionDirectiveAnalyzer;

impl SectionDirectiveAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        name: &String,
        content: &SectionDirectiveContent,
        position: &Position,
    ) -> Result<()> {
        match content {
            SectionDirectiveContent::Text(text) => {
                analyzer.analyze(&Node::Text(text.to_owned()))?
            }
            SectionDirectiveContent::RustExprSimple(expr, is_escaped) => analyzer.analyze(
                &Node::RustExprSimple(expr.to_owned(), *is_escaped, position.to_owned()),
            )?,
        };

        analyzer
            .sections
            .insert(name.to_owned(), position.to_owned());

        Ok(())
    }
}
