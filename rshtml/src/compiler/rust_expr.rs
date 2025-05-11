use crate::compiler::Compiler;
use proc_macro2::TokenStream;
use quote::quote;
use crate::Node;
use std::str::FromStr;

pub struct RustExprCompiler;

impl RustExprCompiler {
    pub fn compile(compiler: &mut Compiler, exprs: &Vec<(String, Vec<Node>)>) -> TokenStream {
        let mut ts = TokenStream::new();

        for (expr, inner_nodes) in exprs {
            let mut inner_ts = TokenStream::new();
            for inner_node in inner_nodes {
                let its = compiler.compile(inner_node);
                inner_ts.extend(quote! {#its;});
            }

            let expr_code = TokenStream::from_str(expr).unwrap();

            ts.extend(quote! { #expr_code { #inner_ts; } });
        }

        ts
    }
}
