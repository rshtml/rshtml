use syn::{Block, parse::Parser};

use crate::{analyzer::Analyzer, diagnostic::Level, position::Position};

pub struct RustBlockAnalyzer;

impl RustBlockAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer, content: &str, position: &Position) {
        // TODO: analyze this it span gives 0
        if let Err(e) = Block::parse_within.parse_str(content) {
            let span = e.span();
            let start = span.start();
            let end = span.end();

            let range = span.byte_range();

            let byte_start = position.byte_positions().0;
            let byte_positions = (byte_start + range.start, byte_start + range.end);

            let line_start = (start.line + position.0.0).saturating_sub(1);
            let line_end = (end.line + position.0.0).saturating_sub(1);

            let col_start = if start.line == 1 {
                position.0.1 + start.column
            } else {
                start.column + 1
            };
            let col_end = if end.line == 1 {
                position.0.1 + end.column
            } else {
                end.column + 1
            };

            let pos = Position((line_start, col_start), (line_end, col_end), byte_positions);

            analyzer.diagnostic(
                &pos,
                "invalid rust code block",
                &[],
                &e.to_string(),
                1,
                Level::Caution,
            );
        }
    }
}
