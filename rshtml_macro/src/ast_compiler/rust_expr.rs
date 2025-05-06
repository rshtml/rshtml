use crate::ast_compiler::compile_ast;
use proc_macro2::TokenStream;
use quote::quote;
use rshtml::Node;
use std::str::FromStr;

pub struct RustExprCompiler;

impl RustExprCompiler {
    pub fn compile(exprs: &Vec<(String, Vec<Node>)>) -> TokenStream {
        let mut ts = TokenStream::new();

        for (expr, inner_nodes) in exprs {
            let mut inner_ts = TokenStream::new();
            for inner_node in inner_nodes {
                let its = compile_ast(inner_node);
                inner_ts.extend(quote! {#its;});
            }

            let expr_code = TokenStream::from_str(expr).unwrap();

            ts.extend(quote! { #expr_code { #inner_ts; } });
        }

        ts
    }
}
