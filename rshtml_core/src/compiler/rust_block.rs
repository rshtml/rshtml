use crate::compiler::Compiler;
use crate::node::{RustBlockContent, TextLineItem};
use crate::Node;
use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

pub struct RustBlockCompiler;

impl RustBlockCompiler {
    pub fn compile(
        compiler: &mut Compiler,
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
                RustBlockContent::TextLine(items) => {
                    for item in items {
                        match item {
                            TextLineItem::Text(text) => {
                                let t_ts = compiler.compile(&Node::Text(text.clone()))?;
                                token_stream.extend(quote! {#t_ts});
                            }
                            TextLineItem::RustExprSimple(expr, is_escaped) => {
                                let rxs_ts = compiler
                                    .compile(&Node::RustExprSimple(expr.clone(), *is_escaped))?;
                                token_stream.extend(quote! {#rxs_ts});
                            }
                        }
                    }
                }
                RustBlockContent::NestedBlock(nested_contents) => {
                    let nested_ts = Self::compile(compiler, nested_contents)?;
                    token_stream.extend(quote! { {#nested_ts} });
                }
            }
        }

        Ok(token_stream)
    }
}
