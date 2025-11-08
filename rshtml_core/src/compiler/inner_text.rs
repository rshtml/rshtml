use crate::compiler::Compiler;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::ops::AddAssign;

pub struct InnerTextCompiler;

impl InnerTextCompiler {
    pub fn compile(compiler: &mut Compiler, inner_text: String) -> Result<TokenStream> {
        compiler.text_size.add_assign(inner_text.len());
        Ok(quote! { write!(__f__, "{}", #inner_text)?; })
    }
}
