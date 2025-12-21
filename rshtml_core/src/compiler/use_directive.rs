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
        _name: String,
        _path: PathBuf,
        component: Node,
        _position: Position,
    ) -> Result<TokenStream> {
        compiler.compile(component)
    }
}
