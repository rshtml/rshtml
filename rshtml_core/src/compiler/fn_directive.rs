pub struct FnDirectiveCompiler;

use crate::{compiler::Compiler, node::Node, position::Position};
use anyhow::{Result, anyhow};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
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

        let mut body_ts = TokenStream::new();
        for b in body {
            let ts = compiler.compile(b)?;
            body_ts.extend(ts);
        }

        let fn_name = format_ident!("{}", &name);

        let fn_sign_ts = quote! { fn #fn_name(&self, __f__: &mut dyn ::std::fmt::Write, #args) -> std::fmt::Result; };
        let fn_body_ts = quote! { fn #fn_name(&self, __f__: &mut dyn ::std::fmt::Write, #args) -> std::fmt::Result {#body_ts Ok(())} };

        compiler
            .components
            .entry(compiler.component_name.to_owned())
            .and_modify(|component_data| {
                component_data.fns.push((fn_sign_ts, fn_body_ts));
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
}
