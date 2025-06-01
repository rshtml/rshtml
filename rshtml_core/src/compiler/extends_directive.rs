use crate::Node;
use crate::compiler::Compiler;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;

pub struct ExtendsDirectiveCompiler;

impl ExtendsDirectiveCompiler {
    pub fn compile(compiler: &mut Compiler, path: &PathBuf, layout: &Box<Node>) -> Result<TokenStream> {
        compiler.layout_directive = path.clone();
        compiler.layout = Some(*layout.clone());

        Ok(quote! {})
    }
}
