use crate::Node;
use crate::compiler::Compiler;
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
