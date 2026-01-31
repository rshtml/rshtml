use std::{collections::HashSet, path::PathBuf};

#[derive(Debug, Default)]
pub struct Context {
    pub text_size: usize,
    pub fn_name: String,
    pub template_params: Vec<(String, String)>,
    pub use_directives: HashSet<UseDirective>,
    pub struct_fields: Vec<String>,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct UseDirective {
    pub name: String,
    pub path: PathBuf,
    pub fn_name: String,
}
