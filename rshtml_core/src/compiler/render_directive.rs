use crate::{compiler::Compiler, position::Position};
use anyhow::Result;
use proc_macro2::TokenStream;

pub struct RenderDirectiveCompiler;

impl RenderDirectiveCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        name: String,
        position: Position,
    ) -> Result<TokenStream> {
        let mut token_stream = TokenStream::new();

        if let Some(section) = compiler.sections.get(&name) {
            token_stream.extend(section.clone());
            token_stream = compiler.with_info(token_stream, position);
        }

        Ok(token_stream)
    }
}
