use crate::{compiler::Compiler, node::Position};
use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use std::str::FromStr;

pub struct RustExprParenCompiler;

impl RustExprParenCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        expr: &str,
        is_escaped: &bool,
        position: &Position,
    ) -> Result<TokenStream> {
        let expr_ts = TokenStream::from_str(expr).map_err(|err| anyhow!("Lex Error: {}", err))?;

        Ok(compiler.escape_or_raw(expr_ts, is_escaped))
    }
}
