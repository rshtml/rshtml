use std::path::Path;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Position(pub (usize, usize), pub (usize, usize), pub (usize, usize)); // start: (line, col), end: (line, col), (start_byte, end_byte)

impl Position {
    pub fn as_info(&self, file: &Path) -> String {
        let positions = if self.0.0 == self.1.0 && self.0.1 == self.1.1 {
            format!("{}:{}", self.0.0, self.0.1)
        } else {
            format!("{}:{}-{}:{}", self.0.0, self.0.1, self.1.0, self.1.1)
        };

        format!("{}:{}", file.display(), positions)
    }
}

impl From<(&str, usize)> for Position {
    fn from((text, offset): (&str, usize)) -> Self {
        let loc = byte_to_line_col(text, offset);
        Position(loc, loc, (offset, offset))
    }
}

impl From<(&str, usize, usize)> for Position {
    fn from((text, start_byte, end_byte): (&str, usize, usize)) -> Self {
        let start_loc = byte_to_line_col(text, start_byte);
        let range_slice = &text[start_byte..end_byte];
        let end_loc = apply_offset_to_pos(range_slice, start_loc.0, start_loc.1);

        Position(start_loc, end_loc, (start_byte, end_byte))
    }
}

fn byte_to_line_col(source: &str, byte_offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (i, ch) in source.char_indices() {
        if i >= byte_offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else if ch == '\r' {
            continue;
        } else {
            col += 1;
        }
    }
    (line, col)
}

fn apply_offset_to_pos(slice: &str, mut line: usize, mut col: usize) -> (usize, usize) {
    for c in slice.chars() {
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}
