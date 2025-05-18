pub fn functions(layout: String, sections: Vec<String>) -> Functions {
    Functions { layout, sections }
}

pub struct Functions {
    pub layout: String,
    sections: Vec<String>,
}

impl Functions {
    pub fn is_section_defined(&self, section_name: &str) -> bool {
        self.sections.contains(&section_name.to_string())
    }
}
