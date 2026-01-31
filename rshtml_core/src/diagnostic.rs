use crate::position::Position;
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct Diagnostic<'a>(pub &'a str);

impl<'a> Diagnostic<'a> {
    pub fn message(
        &self,
        path: &Path,
        position: &Position,
        title: &str,
        info: &str,
        name_len: usize,
    ) -> String {
        let Some(source_snippet) = self.source_first_line(position) else {
            return String::new();
        };
        let line = position.0.0;
        let left_pad = line.to_string().len();

        let lp = " ".repeat(left_pad);
        let file_info = self.files_to_info(path, position);
        let info = if info.is_empty() {
            "".to_string()
        } else {
            let hyphen = "^".repeat(name_len);
            format!("{lp} | {hyphen} {info}\n")
        };

        let lp = " ".repeat(left_pad - line.to_string().len());
        let source_snippet = source_snippet
            .trim_matches(|c| c == ' ' || c == '\t')
            .to_string();
        let source = format!("{lp}{line} | {source_snippet}\n");

        let title = if !title.is_empty() {
            format!("{title}\n")
        } else {
            String::new()
        };

        let lp = " ".repeat(left_pad);
        format!("{title}{lp} --> {file_info}\n{lp} |\n{source}{info}{lp} |",)
    }

    fn source_first_line(&self, position: &Position) -> Option<&str> {
        self.0.lines().nth((position.0).0.saturating_sub(1))
    }

    fn files_to_info(&self, path: &Path, position: &Position) -> String {
        position.as_info(path)
    }
}
