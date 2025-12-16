use syn::{Block, parse::Parser};

use crate::{analyzer::Analyzer, diagnostic::Level, position::Position};

pub struct RustBlockAnalyzer;

impl RustBlockAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer, content: &str, position: &Position) {
        if let Err(e) = Block::parse_within.parse_str(content) {
            let start = e.span().start();
            let end = e.span().end();

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

            let pos = Position((line_start, col_start), (line_end, col_end));

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
