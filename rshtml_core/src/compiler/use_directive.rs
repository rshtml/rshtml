use crate::Node;
use crate::compiler::Compiler;
use crate::position::Position;
use anyhow::Result;
use proc_macro2::TokenStream;
use std::path::PathBuf;

pub struct UseDirectiveCompiler;

impl UseDirectiveCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        name: String,
        path: PathBuf,
        component: Node,
        position: Position,
    ) -> Result<TokenStream> {
        if let Some(c) = compiler.components.get_mut(&compiler.component_path) {
            c.use_directives.push((path, name.to_owned(), position));
        }

        compiler.compile(component)
    }
}
