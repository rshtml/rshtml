use crate::{compiler::Compiler, position::Position};
use anyhow::{Result, anyhow};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, parse_quote, parse_str, visit_mut::VisitMut};

pub struct ExprCompiler<'a>(&'a Vec<String>, bool);

impl<'a> ExprCompiler<'a> {
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

        let mut visitor = ExprCompiler(&component.fn_names, false);
        visitor.visit_expr_mut(&mut expression);
        let is_fn = visitor.1;

        let expr_ts = if is_fn {
            quote!(#expression;)
        } else {
            let file = compiler
                .files
                .iter()
                .last()
                .map(|x| x.0.as_str())
                .unwrap_or("<unknown>");

            let message = compiler.diagnostic.caution(
                file,
                &position,
                "attempt to use an expression that does not implement the Display trait.",
                &[],
                "this expression does not implement the Display trait.",
                expr.len(),
            );
            Self::escape_or_raw(quote!(#expression), is_escaped, &message)
        };
        // TODO: A caution should be given because display implementation is not being used instead of message.

        let expr_ts = compiler.with_info(expr_ts, position, None);

        Ok(expr_ts)
    }

    fn escape_or_raw(expr_ts: TokenStream, is_escaped: bool, message: &str) -> TokenStream {
        if is_escaped {
            quote! { ::rshtml::F(&(#expr_ts)).render(&mut ::rshtml::EscapingWriter { inner: __f__ }, #message)?; }
        } else {
            quote! { ::rshtml::F(&(#expr_ts)).render(__f__, #message)?; }
        }
    }
}

impl<'a> VisitMut for ExprCompiler<'a> {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Call(expr_call) = node
            && let Expr::Path(ref func_path) = *expr_call.func
            && func_path.path.segments.len() == 1
        {
            let ident = &func_path.path.segments[0].ident;

            if self.0.contains(&ident.to_string()) {
                let args = &expr_call.args;

                *node = parse_quote! {self.#ident(__f__, #args)?};
                self.1 = true;
            }
        }
    }
}
