use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Position(pub (usize, usize), pub (usize, usize)); // start: (line, col), end: (line, col)

impl From<&Pair<'_, Rule>> for Position {
    fn from(value: &Pair<Rule>) -> Self {
        Self(
            value.as_span().start_pos().line_col(),
            value.as_span().end_pos().line_col(),
        )
    }
}

impl Position {
    pub fn as_info(&self, files: &[&str]) -> String {
        let positions = format!("{}:{}-{}:{}", self.0 .0, self.0 .1, self.1 .0, self.1 .1);

        if files.is_empty() {
            return positions;
        }

        format!("{}::{}", files.join(" -> "), positions)
    }
}
