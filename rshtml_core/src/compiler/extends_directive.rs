use crate::Node;
use crate::compiler::Compiler;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::path::Path;

pub struct ExtendsDirectiveCompiler;

impl ExtendsDirectiveCompiler {
    pub fn compile(compiler: &mut Compiler, path: &Path, layout: &Node) -> Result<TokenStream> {
        compiler.layout_directive = path.to_path_buf();
        compiler.layout = Some(layout.clone());

        Ok(quote! {})
    }
}
