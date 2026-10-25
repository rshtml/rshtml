use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    ops::Range,
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Clone)]
pub struct UseDirective {
    pub name: String,
    pub path: PathBuf,
    pub fn_name: String,
    pub span: Range<usize>,
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
    pub fn_name: String,
    pub use_directives: HashSet<UseDirective>,
    pub full_path: PathBuf,
}

#[derive(Debug)]
pub enum Op {
    TemplateParams(Range<usize>),
    TemplateParam((Range<usize>, Range<usize>)), // remove self from name
    RustCode(Range<usize>),
    Text(Range<usize>), //@@ ->  @ must do
    OpenBrace(Range<usize>),
    CloseBrace(Range<usize>),
    ChildContent(Range<usize>),
    ContinueOrBreak(Range<usize>),
    ExprSimple(Range<usize>, bool),
    ExprParen(Range<usize>, bool),
    ComponentStartName(Range<usize>), // missing use directive control
    ComponentEnd,
    ComponentBodyStart,
    ComponentAttribute(Range<usize>, Option<Range<usize>>),
}

#[derive(Debug)]
pub struct Context<'a> {
    pub info: Info,
    pub ops: Vec<Op>,
    pub base_dir: &'a Path,
}

impl<'a> Context<'a> {
    pub fn new(base_dir: &'a Path) -> Self {
        Context {
            info: Info::default(),
            base_dir,
            ops: Vec::new(),
        }
    }
}
