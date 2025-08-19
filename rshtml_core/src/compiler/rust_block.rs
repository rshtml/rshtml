use crate::compiler::Compiler;
use crate::node::RustBlockContent;
use anyhow::{Result, anyhow};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

pub struct RustBlockCompiler;

impl RustBlockCompiler {
    pub fn compile(
        _compiler: &mut Compiler,
        contents: &Vec<RustBlockContent>,
    ) -> Result<TokenStream> {
        let mut token_stream = TokenStream::new();

        for content in contents {
            match content {
                RustBlockContent::Code(code) => {
                    let code_ts =
                        TokenStream::from_str(code).map_err(|err| anyhow!("Lex Error: {}", err))?;
                    token_stream.extend(quote! { #code_ts });
                }
                RustBlockContent::NestedBlock(nested_contents) => {
                    let nested_ts = Self::compile(_compiler, nested_contents)?;
                    token_stream.extend(quote! { {#nested_ts} });
                }
            }
        }

        Ok(token_stream)
    }
}
