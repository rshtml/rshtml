use crate::escape::escape;
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

pub struct RustExprSimpleCompiler;

impl RustExprSimpleCompiler {
    pub fn compile(expr: &str) -> TokenStream {
        let expr_ts = TokenStream::from_str(expr).unwrap();

        let escaped = escape(quote! {(#expr_ts)});

        quote! {#escaped}
    }
}
