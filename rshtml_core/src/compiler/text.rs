use crate::compiler::Compiler;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::ops::AddAssign;

pub struct TextCompiler;

impl TextCompiler {
    pub fn compile(compiler: &mut Compiler, text: &String) -> Result<TokenStream> {
        compiler.text_size.add_assign(text.len());
        Ok(quote! { write!(__f__, "{}", #text)?; })
    }
}
