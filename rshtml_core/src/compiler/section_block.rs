use crate::Node;
use crate::compiler::Compiler;
use crate::position::Position;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

pub struct SectionBlockCompiler;

impl SectionBlockCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        name: String,
        content: Vec<Node>,
        _position: Position,
    ) -> Result<TokenStream> {
        let mut token_stream = TokenStream::new();

        for node in content {
            let ts = compiler.compile(node)?;
            token_stream.extend(quote! {#ts});
        }

        compiler.sections.insert(name, token_stream);

        Ok(quote! {})
    }
}
