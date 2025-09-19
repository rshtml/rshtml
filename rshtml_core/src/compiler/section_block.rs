use crate::compiler::Compiler;
use crate::node::Position;
use crate::Node;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

pub struct SectionBlockCompiler;

impl SectionBlockCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        name: &str,
        content: &Vec<Node>,
        position: &Position,
    ) -> Result<TokenStream> {
        let mut token_stream = TokenStream::new();

        for node in content {
            let ts = compiler.compile(node)?;
            token_stream.extend(quote! {#ts});
        }

        compiler.sections.insert(name.to_owned(), token_stream);

        Ok(quote! {})
    }
}
