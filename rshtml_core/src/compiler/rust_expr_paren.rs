use crate::compiler::Compiler;
use proc_macro2::TokenStream;
use std::str::FromStr;

pub struct RustExprParenCompiler;

impl RustExprParenCompiler {
    pub fn compile(compiler: &mut Compiler, expr: &str, is_escaped: &bool) -> TokenStream {
        let expr_ts = TokenStream::from_str(expr).unwrap();

        compiler.escape_or_raw(expr_ts, is_escaped)
    }
}
