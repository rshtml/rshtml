use crate::Node;
use crate::compiler::Compiler;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;

pub struct UseDirectiveCompiler;

impl UseDirectiveCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        name: String,
        _path: PathBuf,
        component: Node,
    ) -> Result<TokenStream> {
        if !compiler.components.contains_key(&name) {
            let component_ts = compiler.compile(component)?;
            compiler.components.insert(name, component_ts);
        }

        Ok(quote! {})
    }
}
