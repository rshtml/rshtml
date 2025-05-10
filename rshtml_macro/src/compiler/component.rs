use crate::compiler::Compiler;
use proc_macro2::TokenStream;
use quote::quote;
use rshtml::Node;
use rshtml::node::{ComponentParameter, ComponentParameterValue};
use std::str::FromStr;

pub struct ComponentCompiler;

impl ComponentCompiler {
    pub fn compile(compiler: &mut Compiler, name: &str, parameters: &Vec<ComponentParameter>, body: &Vec<Node>) -> TokenStream {
        let component_node = compiler.components.get(name).unwrap_or_else(|| panic!("Component {} not found", name));
        let component_node = component_node.clone();

        let mut token_stream = TokenStream::new();

        for parameter in parameters {
            let name_ts = TokenStream::from_str(&parameter.name).unwrap();

            let parameter_ts = match &parameter.value {
                ComponentParameterValue::Bool(value) => quote! {let #name_ts = #value;},
                ComponentParameterValue::Number(value) => quote! {let #name_ts = #value;},
                ComponentParameterValue::String(value) => quote! {let #name_ts = #value;},
                ComponentParameterValue::RustExprParen(value) => {
                    let expr_ts = TokenStream::from_str(value).unwrap();
                    quote! {let #name_ts = #expr_ts;}
                }
                ComponentParameterValue::RustExprSimple(value) => {
                    let expr_ts = TokenStream::from_str(value).unwrap();
                    quote! {let #name_ts = #expr_ts;}
                }
                ComponentParameterValue::Block(value) => {
                    let block_ts = compiler.compile(&Node::Template(value.clone()));
                    quote! {
                        let mut #name_ts = String::new();
                        (|f: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {#block_ts Ok(())})(&mut #name_ts)?;
                    }
                }
            };

            token_stream.extend(parameter_ts);
        }

        let body_ts = compiler.compile(&Node::Template(body.clone()));
        let body_ts = quote! {let mut child_content = |f: &mut ::std::fmt::Formatter<'_>| -> ::std::fmt::Result {#body_ts  Ok(())};};

        token_stream.extend(body_ts);

        let component_ts = compiler.compile(&component_node);

        token_stream.extend(component_ts);

        quote! {{ #token_stream }}
    }
}
