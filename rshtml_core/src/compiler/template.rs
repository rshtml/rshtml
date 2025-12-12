use anyhow::Result;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::{
    compiler::{Compiler, Component},
    node::Node,
    position::Position,
};

pub struct TemplateCompiler;

impl TemplateCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        file: String,
        name: String,
        nodes: Vec<Node>,
        position: Position,
    ) -> Result<TokenStream> {
        if !file.is_empty() {
            compiler.files.push((file.clone(), position.clone()));
        }

        let fn_name = Ident::new(&compiler.generate_fn_name(&name), Span::call_site());

        let fn_call_ts = if compiler.is_root {
            compiler.is_root = false;
            let root_component_ts = quote! {self.#fn_name(__f__, |__f__: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {Ok(())})?;};

            Ok(quote! {#root_component_ts})
        } else {
            Ok(quote! {})
        };

        if !compiler.components.contains_key(&name) {
            let prev_component_name = compiler.component_name.to_owned();
            compiler.component_name = name.to_owned();

            compiler
                .components
                .insert(name.to_owned(), Component::new(fn_name.to_owned()));

            let mut token_stream = TokenStream::new();
            for node in nodes {
                let ts = compiler.compile(node)?;
                token_stream.extend(quote! {#ts});
            }

            let component_ts = token_stream.to_owned();

            if let Some(component_data) = compiler.components.get_mut(&name) {
                let struct_name = &compiler.struct_name;
                let (impl_generics, type_generics, where_clause) =
                    compiler.struct_generics.split_for_impl();

                let fns = &component_data.fns;
                let fn_closures = &component_data.fn_closures;
                let args = component_data.props_to_ts()?;

                let component_ts = quote! {
                    mod #fn_name {
                        use super::*;
                        use ::rshtml::traits::Render;
                        use ::rshtml::traits::Fx;

                        impl #impl_generics super::#struct_name #type_generics #where_clause {
                            pub fn #fn_name(&self,
                                __f__: &mut dyn ::std::fmt::Write,
                                child_content: impl Fn(&mut dyn ::std::fmt::Write) -> ::std::fmt::Result,
                                #args) -> ::std::fmt::Result {#(#fn_closures)* #component_ts  Ok(())}

                            #(#fns)*
                        }
                    }
                };

                component_data.token_stream = component_ts.to_owned();
            }

            compiler.component_name = prev_component_name;
        }

        if !file.is_empty() {
            compiler.files.pop();
        }

        fn_call_ts
    }
}
