use crate::compiler::Compiler;
use anyhow::{Result, anyhow};
use proc_macro2::TokenStream;
use std::str::FromStr;

pub struct RustExprParenCompiler;

impl RustExprParenCompiler {
    pub fn compile(compiler: &mut Compiler, expr: &str, is_escaped: &bool) -> Result<TokenStream> {
        let expr_ts = TokenStream::from_str(expr).map_err(|err| anyhow!("Lex Error: {}", err))?;

        Ok(compiler.escape_or_raw(expr_ts, is_escaped))
    }
}
