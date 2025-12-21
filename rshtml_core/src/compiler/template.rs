use crate::{
    compiler::{Compiler, Component},
    node::Node,
    position::Position,
};
use anyhow::Result;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::path::PathBuf;
use syn::Ident;

pub struct TemplateCompiler;

impl TemplateCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        path: PathBuf,
        name: String,
        fn_names: Vec<String>,
        nodes: Vec<Node>,
        position: Position,
    ) -> Result<TokenStream> {
        compiler.files.push((path.clone(), position.clone()));

        let fn_name = Ident::new(&compiler.generate_fn_name(&name), Span::call_site());

        let fn_call_ts = if compiler.is_root {
            compiler.is_root = false;
            let root_component_ts = quote! {self.#fn_name(__f__, |__f__: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {Ok(())})?;};

            Ok(quote! {#root_component_ts})
        } else {
            Ok(quote! {})
        };

        if !compiler.components.contains_key(&path) {
            let prev_component_path = compiler.component_path.to_owned();
            compiler.component_path = path.to_owned();

            compiler.components.insert(
                path.to_owned(),
                Component::new(fn_name.to_owned(), fn_names),
            );

            let mut token_stream = TokenStream::new();
            for node in nodes {
                let ts = compiler.compile(node)?;
                token_stream.extend(quote! {#ts});
            }

            let component_ts = token_stream.to_owned();

            if let Some(component_data) = compiler.components.get_mut(&path) {
                let struct_name = &compiler.struct_name;
                let (impl_generics, type_generics, where_clause) =
                    compiler.struct_generics.split_for_impl();

                let (fn_signs, fn_bodies): (Vec<&TokenStream>, Vec<&TokenStream>) =
                    component_data.fns.iter().map(|(k, v)| (k, v)).unzip();
                let args = component_data.params_to_ts()?;

                let component_fns = if component_data.fns.is_empty() {
                    quote! {}
                } else {
                    quote! {
                        trait __rshtml__fns {
                            #(#fn_signs)*
                        }
                        impl #impl_generics __rshtml__fns for #struct_name #type_generics #where_clause {
                            #(#fn_bodies)*
                        }
                    }
                };

                let component_ts = quote! {
                fn #fn_name(&self,
                    __f__: &mut dyn ::std::fmt::Write,
                    child_content: impl Fn(&mut dyn ::std::fmt::Write) -> ::std::fmt::Result,
                    #args) -> ::std::fmt::Result {#component_fns #component_ts  Ok(())}
                };

                component_data.token_stream = component_ts.to_owned();
            }

            compiler.component_path = prev_component_path;
        }

        compiler.files.pop();

        fn_call_ts
    }
}
