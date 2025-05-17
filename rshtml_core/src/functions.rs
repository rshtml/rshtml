use crate::compiler::Compiler;

pub struct Functions {
    pub compiler: Compiler,
}

impl Functions {
    pub fn is_section_defined(&self, section_name: &str) -> bool {
        self.compiler.sections.contains_key(section_name)
    }
}
