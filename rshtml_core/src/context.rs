use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct UseDirective {
    pub name: String,
    pub path: PathBuf,
    pub fn_name: String,
}

#[derive(Debug, Default, Clone)]
pub struct Info {
    pub text_size: usize,
    pub fn_name: String,
    pub template_params: Vec<(String, String)>,
    pub use_directives: HashSet<UseDirective>,
}

#[derive(Debug)]
pub struct Context<'a> {
    pub info: Info,
    pub struct_fields: &'a [String],
    pub base_dir: &'a Path,
    pub path: &'a Path,
    pub source: &'a str,
}

impl<'a> Context<'a> {
    pub fn new(
        path: &'a Path,
        source: &'a str,
        base_dir: &'a Path,
        struct_fields: &'a [String],
    ) -> Self {
        Context {
            info: Info::default(),
            struct_fields,
            base_dir,
            path,
            source,
        }
    }
}
