use crate::{debug, position::Position};
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Clone)]
pub struct UseDirective {
    pub name: String,
    pub path: PathBuf,
    pub fn_name: String,
    pub position: Position,
}

impl PartialEq for UseDirective {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}
impl Eq for UseDirective {}
impl Hash for UseDirective {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Info {
    pub text_size: usize,
    pub fn_name: String,
    pub template_params: Vec<(String, String)>,
    pub use_directives: HashSet<UseDirective>,

    pub debug: debug::Debug,
}

#[derive(Debug)]
pub struct Context<'a> {
    pub info: Info,
    pub struct_fields: &'a [String],
    pub base_dir: &'a Path,
    pub path: &'a Path,
    pub source: &'a str,
    pub last_use_directive_point: usize,
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
            last_use_directive_point: 0,
        }
    }
}
