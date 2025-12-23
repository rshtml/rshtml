use crate::Node;
use crate::compiler::Compiler;
use crate::node::{ComponentParameter, ComponentParameterValue};
use crate::position::Position;
use anyhow::{Result, anyhow};
use proc_macro2::TokenStream;
use quote::quote;
use std::ops::AddAssign;
use std::str::FromStr;

pub struct ComponentCompiler;

impl ComponentCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        name: String,
        parameters: Vec<ComponentParameter>,
        body: Vec<Node>,
        position: Position,
    ) -> Result<TokenStream> {
        let use_directive_path = compiler
            .components
            .get(&compiler.component_path)
            .and_then(|c| {
                c.use_directives
                    .iter()
                    .find_map(|(p, n, _)| (*n == name).then(|| p.to_owned()))
            })
            .ok_or(anyhow!("Component {} not found", name))?;

        let (fn_name, args) = compiler
            .components
            .get(&use_directive_path)
            .map(|c| (c.fn_name.to_owned(), c.param_names_to_ts()))
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
                    let mut block_ts = TokenStream::new();
                    for v in value {
                        let ts = compiler.compile(v)?;
                        block_ts.extend(ts);
                    }
                    quote! {let #name_ts = ::rshtml::F::<_, false>(|__f__: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {#block_ts Ok(())});}
                }
            };

            token_stream.extend(parameter_ts);
        }

        let mut body_ts = TokenStream::new();
        for b in body {
            let ts = compiler.compile(b)?;
            body_ts.extend(ts);
        }

        let body_ts = quote! {let child_content = |__f__: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {#body_ts  Ok(())};};

        token_stream.extend(body_ts);

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
