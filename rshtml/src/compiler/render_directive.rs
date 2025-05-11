use crate::compiler::Compiler;
use proc_macro2::TokenStream;

pub struct RenderDirectiveCompiler;

impl RenderDirectiveCompiler {
    pub fn compile(compiler: &mut Compiler, name: &str) -> TokenStream {
        let mut token_stream = TokenStream::new();

        if let Some(section) = compiler.sections.get(name) {
            token_stream.extend(section.clone());
        }

        token_stream
    }
}
