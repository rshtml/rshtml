pub fn functions(layout: String, sections: Vec<String>) -> Functions {
    Functions::new(layout, sections)
}

pub struct Functions {
    pub layout: String,
    sections: Vec<String>,
}

impl Functions {
    fn new(layout: String, sections: Vec<String>) -> Self {
        Self { layout, sections }
    }

    pub fn is_section_defined(&self, section_name: &str) -> bool {
        self.sections.contains(&section_name.to_string())
    }

    pub fn t(&self, word: &str) -> &str {
        todo!()
    }

    pub fn json<T>(&self, t: T) -> &str {
        todo!()
    }
}
