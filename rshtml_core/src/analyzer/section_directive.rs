use crate::{
    analyzer::Analyzer,
    node::{Node, SectionDirectiveContent},
    position::Position,
};

pub struct SectionDirectiveAnalyzer;

impl SectionDirectiveAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        name: &String,
        content: &SectionDirectiveContent,
        position: &Position,
    ) {
        match content {
            SectionDirectiveContent::Text(text) => analyzer.analyze(&Node::Text(text.to_owned())),
            SectionDirectiveContent::RustExprSimple(expr, is_escaped) => analyzer.analyze(
                &Node::RustExprSimple(expr.to_owned(), *is_escaped, position.to_owned()),
            ),
        };

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
