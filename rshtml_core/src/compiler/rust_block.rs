use crate::compiler::Compiler;
use anyhow::{Result, anyhow};
use proc_macro2::TokenStream;
use std::str::FromStr;

pub struct RustBlockCompiler;

impl RustBlockCompiler {
    pub fn compile(_compiler: &mut Compiler, content: &str) -> Result<TokenStream> {
        let mut token_stream = TokenStream::new();

        let code_ts =
            TokenStream::from_str(content).map_err(|err| anyhow!("Lex Error: {}", err))?;
        token_stream.extend(code_ts);

        Ok(token_stream)
    }
}
