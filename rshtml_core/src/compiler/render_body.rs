use crate::compiler::Compiler;
use anyhow::Result;
use proc_macro2::TokenStream;

pub struct RenderBodyCompiler;

impl RenderBodyCompiler {
    pub fn compile(compiler: &mut Compiler) -> Result<TokenStream> {
        let mut token_stream = TokenStream::new();

        if let Some(section_body) = &compiler.section_body {
            token_stream.extend(section_body.clone());
        }

        Ok(token_stream)
    }
}
