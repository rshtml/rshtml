use crate::compiler::Compiler;
use proc_macro2::TokenStream;
use quote::quote;
use rshtml::Node;
use std::path::PathBuf;

pub struct ExtendsDirectiveCompiler;

impl ExtendsDirectiveCompiler {
    pub fn compile(compiler: &mut Compiler, path: &PathBuf, layout: &Box<Node>) -> TokenStream {
        compiler.layout_directive = path.clone();
        compiler.layout = Some(*layout.clone());

        quote! {}
    }
}
