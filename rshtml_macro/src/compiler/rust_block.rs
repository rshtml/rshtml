use crate::compiler::Compiler;
use proc_macro2::TokenStream;
use quote::quote;
use rshtml::Node;
use rshtml::node::RustBlockContent;
use std::str::FromStr;

pub struct RustBlockCompiler;

impl RustBlockCompiler {
    pub fn compile(compiler: &mut Compiler, contents: &Vec<RustBlockContent>) -> TokenStream {
        let mut token_stream = TokenStream::new();

        for content in contents {
            match content {
                RustBlockContent::Code(code) => {
                    let code_ts = TokenStream::from_str(code).unwrap();
                    token_stream.extend(quote! { #code_ts });
                }
                RustBlockContent::TextLine(items) => {
                    for item in items {
                        match item {
                            rshtml::node::TextLineItem::Text(text) => {
                                token_stream.extend(quote! { write!(f, "{}", #text)?; });
                            }
                            rshtml::node::TextLineItem::RustExprSimple(expr) => {
                                let rxs_ts = compiler.compile(&Node::RustExprSimple(expr.clone()));
                                token_stream.extend(quote! {#rxs_ts;});
                            }
                        }
                    }
                }
                RustBlockContent::TextBlock(items) => {
                    for item in items {
                        match item {
                            rshtml::node::TextBlockItem::Text(text) => {
                                token_stream.extend(quote! { write!(f, "{}", #text)?; });
                            }
                            rshtml::node::TextBlockItem::RustExprSimple(expr) => {
                                let rxs_ts = compiler.compile(&Node::RustExprSimple(expr.clone()));
                                token_stream.extend(quote! {#rxs_ts;});
                            }
                        }
                    }
                }
                RustBlockContent::NestedBlock(nested_contents) => {
                    let nested_ts = Self::compile(compiler, nested_contents);
                    token_stream.extend(quote! { {#nested_ts} });
                }
            }
        }

        token_stream
    }
}
