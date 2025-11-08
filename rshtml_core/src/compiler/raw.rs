use crate::compiler::Compiler;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::ops::AddAssign;

pub struct RawCompiler;

impl RawCompiler {
    pub fn compile(compiler: &mut Compiler, body: String) -> Result<TokenStream> {
        compiler.text_size.add_assign(body.len());
        Ok(quote! { write!(__f__, "{}", #body)?; })
    }
}
