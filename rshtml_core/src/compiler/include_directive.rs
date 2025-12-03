use std::path::PathBuf;

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{compiler::Compiler, node::Node};

pub struct IncludeDirectiveCompiler;

impl IncludeDirectiveCompiler {
    pub fn compile(compiler: &mut Compiler, _path: PathBuf, template: Node) -> Result<TokenStream> {
        let token_stream = compiler.compile(template)?;

        Ok(quote! {{#token_stream}})
    }
}
