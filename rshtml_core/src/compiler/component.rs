use crate::compiler::Compiler;
use crate::node::{ComponentParameter, ComponentParameterValue};
use crate::position::Position;
use crate::Node;
use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use quote::quote;
use std::ops::AddAssign;
use std::str::FromStr;

pub struct ComponentCompiler;

impl ComponentCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        name: &str,
        parameters: &Vec<ComponentParameter>,
        body: &[Node],
        position: &Position,
    ) -> Result<TokenStream> {
        let component_node = compiler
            .components
            .get(name)
            .ok_or(anyhow!("Component {} not found", name))?;
        let component_node = component_node.clone();

        let mut token_stream = TokenStream::new();

        for parameter in parameters {
            let name_ts = TokenStream::from_str(&parameter.name)
                .map_err(|err| anyhow!("Lex Error: {}", err))?;

            let parameter_ts = match &parameter.value {
                ComponentParameterValue::Bool(value) => quote! {let #name_ts = #value;},
                ComponentParameterValue::Number(value) => {
                    compiler.text_size.add_assign(value.len());
                    quote! {let #name_ts = #value;}
                }
                ComponentParameterValue::String(value) => {
                    compiler.text_size.add_assign(value.len());
                    quote! {let #name_ts = #value;}
                }
                ComponentParameterValue::RustExprParen(value) => {
                    let expr_ts = TokenStream::from_str(value)
                        .map_err(|err| anyhow!("Lex Error: {}", err))?;
                    quote! {let #name_ts = #expr_ts;}
                }
                ComponentParameterValue::RustExprSimple(value) => {
                    let expr_ts = TokenStream::from_str(value)
                        .map_err(|err| anyhow!("Lex Error: {}", err))?;
                    quote! {let #name_ts = #expr_ts;}
                }
                ComponentParameterValue::Block(value) => {
                    let block_ts = compiler.compile(&Node::Template(
                        String::new(),
                        value.clone(),
                        position.clone(),
                    ))?;
                    quote! {
                        let mut #name_ts = String::new();
                        (|__f__: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {#block_ts Ok(())})(&mut #name_ts)?;
                    }
                }
            };

            token_stream.extend(parameter_ts);
        }

        let body_ts = compiler.compile(&Node::Template(
            String::new(),
            body.to_owned(),
            position.clone(),
        ))?;
        let body_ts = quote! {let child_content = |__f__: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {#body_ts  Ok(())};};

        token_stream.extend(body_ts);

        let component_ts = compiler.compile(&component_node);

        token_stream.extend(component_ts);

        Ok(quote! {{ #token_stream }})
    }
}
