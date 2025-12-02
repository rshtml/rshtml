use crate::compiler::Compiler;
use anyhow::Result;
use proc_macro2::TokenStream;

pub struct RenderDirectiveCompiler;

impl RenderDirectiveCompiler {
    pub fn compile(compiler: &mut Compiler, name: String) -> Result<TokenStream> {
        let mut token_stream = TokenStream::new();

        if let Some(section) = compiler.sections.get(&name) {
            token_stream.extend(section.clone());
        }

        Ok(token_stream)
    }
}
