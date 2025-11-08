use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{compiler::Compiler, node::Node, position::Position};

pub struct TemplateCompiler;

impl TemplateCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        file: String,
        nodes: Vec<Node>,
        position: Position,
    ) -> Result<TokenStream> {
        if !file.is_empty() {
            compiler.files.push((file.clone(), position.clone()));
        }

        let mut token_stream = TokenStream::new();
        for node in nodes {
            let ts = compiler.compile(node)?;
            token_stream.extend(quote! {#ts});
        }

        if !file.is_empty() {
            compiler.files.pop();
        }

        Ok(token_stream)
    }
}
