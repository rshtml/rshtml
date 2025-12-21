use std::path::Path;

use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Position(pub (usize, usize), pub (usize, usize), pub (usize, usize)); // start: (line, col), end: (line, col), (start_byte, end_byte)

impl From<&Pair<'_, Rule>> for Position {
    fn from(value: &Pair<Rule>) -> Self {
        let span = value.as_span();
        Self(
            span.start_pos().line_col(),
            span.end_pos().line_col(),
            (span.start(), span.end()),
        )
    }
}

impl Position {
    pub fn as_info(&self, file: &Path) -> String {
        let positions = format!("{}:{}-{}:{}", self.0.0, self.0.1, self.1.0, self.1.1);

        format!("{}:{}", file.to_str().unwrap_or(""), positions)
    }

    pub fn byte_positions(&self) -> (usize, usize) {
        self.2
    }
}
