use crate::{compiler::Compiler, position::Position};
use anyhow::{Result, anyhow};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, parse_quote, parse_str, visit_mut::VisitMut};

pub struct RustExprSimpleCompiler<'a>(&'a Vec<String>, bool);

impl<'a> RustExprSimpleCompiler<'a> {
    pub fn compile(
        compiler: &mut Compiler,
        expr: String,
        is_escaped: bool,
        position: Position,
    ) -> Result<TokenStream> {
        let mut expression =
            parse_str::<Expr>(&expr).map_err(|err| anyhow!("Expression Parsing Error: {err}"))?;

        let component = compiler
            .components
            .get(&compiler.component_name)
            .ok_or(anyhow!("component not found"))?;

        let mut visitor = RustExprSimpleCompiler(&component.fn_names, false);
        visitor.visit_expr_mut(&mut expression);
        let is_fn = visitor.1;

        let expr_ts = quote!(#expression);

        let expr_ts = if is_fn {
            expr_ts
        } else {
            compiler.escape_or_raw(expr_ts, is_escaped, "message")
        };
        // TODO: A caution should be given because display implementation is not being used instead of message.

        let expr_ts = compiler.with_info(expr_ts, position, None);

        Ok(expr_ts)
    }
}

impl<'a> VisitMut for RustExprSimpleCompiler<'a> {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Call(expr_call) = node
            && let Expr::Path(ref func_path) = *expr_call.func
            && func_path.path.segments.len() == 1
        {
            let ident = &func_path.path.segments[0].ident;

            if self.0.contains(&ident.to_string()) {
                let args = &expr_call.args;

                *node = parse_quote! {{self.#ident(__f__, #args)?;}};
                self.1 = true;
            }
        }
    }
}
