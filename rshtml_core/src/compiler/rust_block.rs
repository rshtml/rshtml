use crate::{compiler::Compiler, position::Position};
use anyhow::{Result, anyhow};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

pub struct RustBlockCompiler;

impl RustBlockCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        content: &str,
        position: &Position,
    ) -> Result<TokenStream> {
        let code_ts =
            TokenStream::from_str(content).map_err(|err| anyhow!("Lex Error: {}", err))?;

        let info_ts = compiler.with_info(TokenStream::new(), position);

        let code_ts = quote! {
             "Rust Code Block Start";
             #info_ts
             #code_ts
             "Rust Code Block End";
        };

        Ok(code_ts)
    }
}
