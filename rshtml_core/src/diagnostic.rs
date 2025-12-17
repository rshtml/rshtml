use crate::position::Position;
use std::collections::HashMap;

pub struct Diagnostic {
    sources: HashMap<String, String>,
}

impl Diagnostic {
    pub fn new(sources: HashMap<String, String>) -> Self {
        Self { sources }
    }

    pub fn message(
        &self,
        file: &str,
        position: &Position,
        title: &str,
        lines: &[usize],
        info: &str,
        name_len: usize,
    ) -> String {
        let (lines, source_snippet, left_pad) = if lines.is_empty() {
            (
                &[(position.0).0] as &[usize],
                self.source_first_line(file, position).unwrap_or_default(),
                ((position.0).0).to_string().len(),
            )
        } else {
            (
                lines,
                self.extract_source_snippet(file, position)
                    .unwrap_or_default(),
                ((position.1).0).to_string().len(),
            )
        };

        let lp = " ".repeat(left_pad);
        let file_info = self.files_to_info(file, position);
        let info = if info.is_empty() {
            "".to_string()
        } else {
            let hyphen = "-".repeat(name_len);
            format!("{lp} | {hyphen} {info}\n")
        };

        let mut source = String::new();

        let first_line = (position.0).0;

        for (i, source_line) in source_snippet.lines().enumerate() {
            let current_line = first_line + i;
            let lp = left_pad - current_line.to_string().len();
            let lp = " ".repeat(lp);

            if lines.contains(&current_line) {
                source.push_str(format!("{lp}{current_line} | {source_line}\n").as_str());
            }
        }

        let title = if !title.is_empty() {
            &format!("{title}\n")
        } else {
            ""
        };

        let lp = " ".repeat(left_pad);
        format!("{title}{lp} --> {file_info}\n{lp} |\n{source}{info}{lp} |",)
    }

    pub fn warning(
        &self,
        file: &str,
        position: &Position,
        title: &str,
        lines: &[usize],
        info: &str,
        name_len: usize,
    ) -> String {
        let yellow = "\x1b[33m";
        let reset = "\x1b[0m";

        let warn = self.message(file, position, title, lines, info, name_len);

        format!("{yellow}warning:{reset} {warn}")
    }

    pub fn caution(
        &self,
        file: &str,
        position: &Position,
        title: &str,
        lines: &[usize],
        info: &str,
        name_len: usize,
    ) -> String {
        let magenta = "\x1b[1;35m";
        let reset = "\x1b[0m";

        let cau = self.message(file, position, title, lines, info, name_len);

        format!("{magenta}caution:{reset} {cau}")
    }

    fn extract_source_snippet(&self, file: &str, position: &Position) -> Option<&str> {
        let source = self.sources.get(file)?;
        let (start, end) = position.byte_positions();
        Some(&source[start..end])
    }

    fn source_first_line(&self, file: &str, position: &Position) -> Option<&str> {
        self.sources
            .get(file)?
            .lines()
            .nth((position.0).0.saturating_sub(1))
            .map(|s| s)
    }

    fn files_to_info(&self, file: &str, position: &Position) -> String {
        position.as_info(file)
    }
}

pub enum Level {
    Warning,
    Caution,
}
