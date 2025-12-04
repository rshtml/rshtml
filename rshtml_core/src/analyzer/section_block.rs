use crate::{analyzer::Analyzer, node::Node, position::Position};

pub struct SectionBlockAnalyzer;

impl SectionBlockAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        name: &String,
        content: &Vec<Node>,
        position: &Position,
    ) {
        for node in content {
            analyzer.analyze(node);
        }

        if !analyzer.no_warn && analyzer.sections.iter().any(|(n, _)| n == name) {
            analyzer.warning(
                position,
                &format!("attempt to redefine section `{name}`"),
                &[],
                &format!("section `{name}` is redefined"),
                "section".len(),
            );
        }

        analyzer
            .sections
            .push((name.to_owned(), position.to_owned()));
    }
}
