use crate::{compiler::Compiler, position::Position};
use anyhow::{Result, anyhow};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

pub struct RustBlockCompiler;

impl RustBlockCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        content: String,
        position: Position,
    ) -> Result<TokenStream> {
        let code_ts =
            TokenStream::from_str(&content).map_err(|err| anyhow!("Lex Error: {}", err))?;

        let code_ts = compiler.with_info(
            code_ts,
            position,
            Some(("Rust Code Block Start", "Rust Code Block End", false)),
        );

        let code_ts = quote! { #code_ts };

        Ok(code_ts)
    }
}
