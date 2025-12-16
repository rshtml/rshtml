use crate::{compiler::Compiler, position::Position};
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

pub struct TemplateParamsCompiler;

impl TemplateParamsCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        params: Vec<(String, String, Position)>,
        _position: Position,
    ) -> Result<TokenStream> {
        compiler
            .components
            .entry(compiler.component_name.to_owned())
            .and_modify(|component_data| {
                component_data
                    .params
                    .extend(params.iter().map(|(param_name, param_type, _)| {
                        (param_name.to_owned(), param_type.to_owned())
                    }))
            });

        Ok(quote! {})
    }
}
