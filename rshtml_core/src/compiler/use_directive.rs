use crate::compiler::Compiler;
use crate::position::Position;
use crate::Node;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::path::Path;

pub struct UseDirectiveCompiler;

impl UseDirectiveCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        name: &String,
        path: &Path,
        component: &Node,
        position: &Position,
    ) -> Result<TokenStream> {
        compiler
            .use_directives
            .push((name.to_string(), path.to_path_buf()));
        compiler
            .components
            .insert(name.to_string(), (*component).clone());

        Ok(quote! {})
    }
}
