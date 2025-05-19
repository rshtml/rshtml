use serde::Serialize;

pub fn functions(layout: String, sections: Vec<String>) -> Functions {
    Functions::new(layout, sections)
}

pub struct Functions {
    pub layout: String,
    sections: Vec<String>,
    pushed_texts: Vec<String>,
}

impl Functions {
    fn new(layout: String, sections: Vec<String>) -> Self {
        Self {
            layout,
            sections,
            pushed_texts: Vec::new(),
        }
    }

    pub fn is_section_defined(&self, section_name: &str) -> bool {
        self.sections.contains(&section_name.to_string())
    }

    pub fn t(&self, word: &str) -> &str {
        todo!()
    }

    pub fn json<T: Serialize>(&self, value: &T) -> String {
        serde_json::to_string(value).unwrap_or(String::new())
    }

    pub fn push(&mut self, text: String) {
        self.pushed_texts.push(text);
    }

    pub fn stack(&self) -> String {
        self.pushed_texts.iter().fold(String::new(), |mut x, y| {
            x.push_str(y);
            x.push('\n');
            x
        })
    }
}
