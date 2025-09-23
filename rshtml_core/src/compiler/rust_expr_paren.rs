use crate::{compiler::Compiler, position::Position};
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

        let expr_ts = compiler.escape_or_raw(expr_ts, is_escaped);

        let expr_ts = compiler.with_info(expr_ts, position);

        Ok(expr_ts)
    }
}
