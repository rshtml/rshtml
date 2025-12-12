pub struct FnDirectiveCompiler;

use crate::{compiler::Compiler, node::Node, position::Position};
use anyhow::{Result, anyhow};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use std::str::FromStr;
use syn::{Ident, Type, parse_str};

impl FnDirectiveCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        name: String,
        params: Vec<(String, String, Position)>,
        body: Vec<Node>,
        _position: Position,
    ) -> Result<TokenStream> {
        let args = Self::params_to_ts(&params)?;
        let arg_names = Self::param_names_to_ts(&params);

        let mut body_ts = TokenStream::new();
        for b in body {
            let ts = compiler.compile(b)?;
            body_ts.extend(ts);
        }

        let fn_name = format_ident!("_{}", &name);
        let name_ts = TokenStream::from_str(&name).map_err(|e| anyhow!("Lex Error: {e}"))?;

        let fn_ts = quote! {
            fn #fn_name(&self, __f__: &mut dyn ::std::fmt::Write, #args) -> std::fmt::Result {#body_ts Ok(())}
        };

        let fn_closure = quote! {let mut #name_ts = |#args| -> ::std::fmt::Result {self.#fn_name(__f__, #arg_names)};};

        compiler
            .components
            .entry(compiler.component_name.to_owned())
            .and_modify(|component_data| {
                component_data.fns.push(fn_ts);
                component_data.fn_closures.push(fn_closure);
            });

        Ok(quote! {})
    }

    fn params_to_ts(params: &Vec<(String, String, Position)>) -> Result<TokenStream> {
        let mut args = Vec::new();

        for (prop_name, prop_type, _) in params {
            let prop_name = Ident::new(prop_name, Span::call_site());
            let prop_type = parse_str::<Type>(prop_type)
                .map_err(|e| anyhow!("Invalid prop type: {prop_type}, {e}"))?;

            args.push(quote! { #prop_name: #prop_type});
        }

        Ok(quote! {#(#args),*})
    }

    fn param_names_to_ts(params: &Vec<(String, String, Position)>) -> TokenStream {
        let args = params
            .iter()
            .map(|prop| Ident::new(&prop.0, Span::call_site()));
        quote! {#(#args),*}
    }
}
