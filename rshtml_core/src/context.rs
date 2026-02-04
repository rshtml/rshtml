use std::{collections::HashSet, path::PathBuf};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct UseDirective {
    pub name: String,
    pub path: PathBuf,
    pub fn_name: String,
}

#[derive(Debug, Default)]
pub struct Context {
    pub text_size: usize,
    pub fn_name: String,
    pub template_params: Vec<(String, String)>,
    pub use_directives: HashSet<UseDirective>,
    pub struct_fields: Vec<String>,
    pub base_dir: PathBuf,
}

impl Context {
    pub fn new(base_dir: PathBuf, struct_fields: Vec<String>) -> Self {
        Context {
            base_dir,
            struct_fields,
            ..Default::default()
        }
    }
}
