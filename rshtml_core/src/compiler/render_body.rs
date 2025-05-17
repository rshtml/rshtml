use crate::compiler::Compiler;
use proc_macro2::TokenStream;

pub struct RenderBodyCompiler;

impl RenderBodyCompiler {
    pub fn compile(compiler: &mut Compiler) -> TokenStream {
        let mut token_stream = TokenStream::new();

        if let Some(section_body) = &compiler.section_body {
            token_stream.extend(section_body.clone());
        }

        token_stream
    }
}
