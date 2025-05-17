use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

pub struct RustExprParenCompiler;

impl RustExprParenCompiler {
    pub fn compile(expr: &str) -> TokenStream {
        let expr_ts = TokenStream::from_str(expr).unwrap();
        quote! {
            write!(f, "{}", #expr_ts)?
        }
    }
}
