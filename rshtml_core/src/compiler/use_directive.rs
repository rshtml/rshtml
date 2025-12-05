use crate::compiler::Compiler;
use crate::position::Position;
use crate::{Node, compiler::Component};
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;

pub struct UseDirectiveCompiler;

impl UseDirectiveCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        name: String,
        path: PathBuf,
        component: Node,
        _position: Position,
    ) -> Result<TokenStream> {
        compiler.use_directives.push((path, name.to_owned()));

        if !compiler.components.contains_key(&name) {
            compiler
                .components
                .insert(name.to_owned(), Component::new());

            let component_ts = compiler.compile(component)?;

            if let Some(component_data) = compiler.components.get_mut(&name) {
                let args = component_data.props_to_ts()?;
                let fn_name = syn::Ident::new(&name, proc_macro2::Span::call_site());

                let component_ts = quote! { fn #fn_name (&mut self,
                __f__: &mut dyn ::std::fmt::Write,
                child_content: impl Fn(&mut dyn ::std::fmt::Write) -> ::std::fmt::Result,
                #args) -> ::std::fmt::Result {#component_ts  Ok(())} };

                component_data.token_stream = component_ts.to_owned();
            }
        }

        Ok(quote! {})
    }
}
