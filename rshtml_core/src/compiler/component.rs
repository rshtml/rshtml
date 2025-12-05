use crate::Node;
use crate::compiler::Compiler;
use crate::node::{ComponentParameter, ComponentParameterValue};
use crate::position::Position;
use anyhow::{Result, anyhow};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::ops::AddAssign;
use std::str::FromStr;
use syn::Ident;

pub struct ComponentCompiler;

impl ComponentCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        name: String,
        parameters: Vec<ComponentParameter>,
        body: Vec<Node>,
        position: Position,
    ) -> Result<TokenStream> {
        let component_data = compiler
            .components
            .get(&name)
            .cloned()
            .ok_or(anyhow!("Component {} not found", name))?;

        let mut token_stream = TokenStream::new();

        for parameter in parameters {
            let name_ts = TokenStream::from_str(&parameter.name)
                .map_err(|err| anyhow!("Lex Error: {}", err))?;

            let parameter_ts = match parameter.value {
                ComponentParameterValue::Bool(value) => quote! {let #name_ts = #value;},
                ComponentParameterValue::Number(value) => {
                    compiler.text_size.add_assign(value.len());
                    let value =
                        TokenStream::from_str(&value).map_err(|e| anyhow!(e.to_string()))?;
                    quote! {let #name_ts = #value;}
                }
                ComponentParameterValue::String(value) => {
                    compiler.text_size.add_assign(value.len());
                    quote! {let #name_ts = #value;}
                }
                ComponentParameterValue::RustExprParen(value) => {
                    let expr_ts = TokenStream::from_str(&value)
                        .map_err(|err| anyhow!("Lex Error: {}", err))?;
                    quote! {let #name_ts = #expr_ts;}
                }
                ComponentParameterValue::RustExprSimple(value) => {
                    let expr_ts = TokenStream::from_str(&value)
                        .map_err(|err| anyhow!("Lex Error: {}", err))?;
                    quote! {let #name_ts = #expr_ts;}
                }
                ComponentParameterValue::Block(value) => {
                    let block_ts =
                        compiler.compile(Node::Template(String::new(), value, position.clone()))?;
                    quote! {let #name_ts = |__f__: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {#block_ts Ok(())};}
                }
            };

            token_stream.extend(parameter_ts);
        }

        let body_ts = compiler.compile(Node::Template(String::new(), body, position.clone()))?;
        let body_ts = quote! {let child_content = |__f__: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {#body_ts  Ok(())};};

        token_stream.extend(body_ts);

        let args = component_data.prop_names_to_ts();
        let fn_name = Ident::new(&name, Span::call_site());
        let component_ts = quote! {self.#fn_name(__f__, child_content, #args)?;};

        token_stream.extend(component_ts);

        let token_stream = compiler.with_info(
            token_stream,
            position,
            Some((
                &format!("Component {} Start", name),
                &format!("Component {} End", name),
                true,
            )),
        );

        Ok(quote! {{ #token_stream }})
    }
}
