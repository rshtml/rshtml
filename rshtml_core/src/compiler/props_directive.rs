use crate::{compiler::Compiler, position::Position};
use anyhow::{Result, anyhow};
use proc_macro2::TokenStream;
use quote::quote;

pub struct PropsDirectiveCompiler;

impl PropsDirectiveCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        props: Vec<(String, String, Position)>,
        _position: Position,
    ) -> Result<TokenStream> {
        if let Some((_, name)) = compiler.use_directives.last() {
            compiler
                .components
                .entry(name.to_owned())
                .and_modify(|component_data| {
                    component_data
                        .props
                        .extend(props.iter().map(|(prop_name, prop_type, _)| {
                            (prop_name.to_owned(), prop_type.to_owned())
                        }))
                });

            return Ok(quote! {});
        }

        return Err(anyhow!("Component not found"));
    }
}
