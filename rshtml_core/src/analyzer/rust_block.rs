use crate::{analyzer::Analyzer, diagnostic::Level, position::Position};
use syn::{Block, parse::Parser};

pub struct RustBlockAnalyzer;

impl RustBlockAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer, content: &str, position: &Position) {
        if let Err(e) = Block::parse_within.parse_str(content) {
            let mut pos = position.clone();
            if let Some((path, _)) = analyzer.files.last()
                && let Some(source) = analyzer.diagnostic.sources.get(path)
            {
                let current_start_byte = pos.2.0;
                let prefix = &source[..current_start_byte];

                let line_start_idx = prefix.rfind('\n').map(|i| i + 1).unwrap_or(0);

                if let Some(at_index_in_slice) =
                    source[line_start_idx..current_start_byte].rfind('@')
                {
                    pos.2.0 = line_start_idx + at_index_in_slice;
                }
            }

            analyzer.diagnostic(
                &pos,
                "invalid rust code block",
                &[position.0.0, position.1.0],
                &e.to_string(),
                0,
                Level::Caution,
            );
        }
    }
}
