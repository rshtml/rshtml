use crate::compiler::Compiler;
use crate::position::Position;
use crate::Node;
use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

pub struct MatchExprCompiler;

impl MatchExprCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        (name, name_position): (&str, &Position),
        arms: &Vec<((String, Position), Vec<Node>)>,
        position: &Position,
    ) -> Result<TokenStream> {
        let mut arms_ts = TokenStream::new();

        for ((arm_name, arm_position), arm_nodes) in arms {
            let mut token_stream = TokenStream::new();
            for node in arm_nodes {
                let ts = compiler.compile(node)?;
                token_stream.extend(quote! {#ts});
            }
            let arm_head =
                TokenStream::from_str(arm_name).map_err(|err| anyhow!("Lex Error: {}", err))?;
            let arm_ts = quote! {
                #arm_head =>  { #token_stream },
            };

            arms_ts.extend(arm_ts);
        }

        let name_head = TokenStream::from_str(name).map_err(|err| anyhow!("Lex Error: {}", err))?;

        Ok(quote! {
           #name_head {
             #arms_ts
           }
        })
    }
}
