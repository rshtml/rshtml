mod component;
mod expr;
mod fn_directive;
mod match_expr;
mod raw;
mod rust_block;
mod rust_expr;
mod template;
mod template_params;
mod text;
mod use_directive;

use crate::Node;
use crate::compiler::component::ComponentCompiler;
use crate::compiler::expr::ExprCompiler;
use crate::compiler::fn_directive::FnDirectiveCompiler;
use crate::compiler::match_expr::MatchExprCompiler;
use crate::compiler::raw::RawCompiler;
use crate::compiler::rust_block::RustBlockCompiler;
use crate::compiler::rust_expr::RustExprCompiler;
use crate::compiler::template::TemplateCompiler;
use crate::compiler::template_params::TemplateParamsCompiler;
use crate::compiler::text::TextCompiler;
use crate::compiler::use_directive::UseDirectiveCompiler;
use crate::diagnostic::Diagnostic;
use crate::position::Position;
use anyhow::Result;
use anyhow::anyhow;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::path::PathBuf;
use syn::Generics;
use syn::Ident;
use syn::Type;
use syn::parse_str;

pub struct Compiler {
    struct_name: Ident,
    struct_generics: Generics,
    components: HashMap<PathBuf, Component>,
    pub text_size: usize,
    pub files: Vec<(PathBuf, Position)>,
    is_root: bool,
    component_path: PathBuf,
    diagnostic: Diagnostic,
}

impl Compiler {
    pub fn new(struct_name: Ident, struct_generics: Generics, diagnostic: Diagnostic) -> Self {
        Compiler {
            struct_name,
            struct_generics,
            components: HashMap::new(),
            text_size: 0,
            files: Vec::new(),
            is_root: false,
            component_path: PathBuf::new(),
            diagnostic,
        }
    }

    pub fn compile(&mut self, node: Node) -> Result<TokenStream> {
        match node {
            Node::Template(path, name, fns, nodes, position) => {
                TemplateCompiler::compile(self, path, name, fns, nodes, position)
            }
            Node::Text(text) => TextCompiler::compile(self, text),
            Node::TemplateParams(params, position) => {
                TemplateParamsCompiler::compile(self, params, position)
            }
            Node::RustBlock(content, position) => {
                RustBlockCompiler::compile(self, content, position)
            }
            Node::Expr(expr, is_escaped, position) => {
                ExprCompiler::compile(self, expr, is_escaped, position)
            }
            Node::MatchExpr(head, arms, position) => {
                MatchExprCompiler::compile(self, head, arms, position)
            }
            Node::RustExpr(exprs, position) => RustExprCompiler::compile(self, exprs, position),
            Node::Component(name, parameters, body, position) => {
                ComponentCompiler::compile(self, name, parameters, body, position)
            }
            Node::ChildContent => Ok(quote! {child_content(__f__)?;}),
            Node::FnDirective(name, params, body, position) => {
                FnDirectiveCompiler::compile(self, name, params, body, position)
            }
            Node::Raw(body) => RawCompiler::compile(self, body),
            Node::UseDirective(name, path, component, position) => {
                UseDirectiveCompiler::compile(self, name, path, *component, position)
            }
            Node::ContinueDirective => Ok(quote! {continue;}),
            Node::BreakDirective => Ok(quote! {break;}),
        }
    }

    pub fn run(&mut self, node: Node) -> Result<TokenStream> {
        self.is_root = true;
        let ts = self.compile(node)?;

        Ok(ts)
    }

    pub fn component_fns(&self) -> TokenStream {
        let mut token_stream = TokenStream::new();
        self.components.values().for_each(|component_data| {
            token_stream.extend(component_data.token_stream.to_owned());
        });

        token_stream
    }

    fn generate_fn_name(&self, name: &str) -> String {
        let mut hash: u64 = 5381;
        for c in name.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u64);
        }

        format!("{}_{:x}", name, hash)
    }

    fn with_info(
        &self,
        expr_ts: TokenStream,
        position: Position,
        infos: Option<(&str, &str, bool)>,
    ) -> TokenStream {
        if cfg!(debug_assertions) {
            let mapping = position.as_info(&self.component_path);

            if let Some((start, end, is_scoped)) = infos {
                if is_scoped {
                    quote! {{#start;#mapping;#expr_ts #end;}}
                } else {
                    quote! {#start;#mapping;#expr_ts #end;}
                }
            } else {
                quote! {{#mapping;#expr_ts}}
            }
        } else {
            expr_ts
        }
    }
}

#[derive(Debug, Clone)]
struct Component {
    fn_name: Ident,
    token_stream: TokenStream,
    params: Vec<(String, String)>,
    fns: Vec<(TokenStream, TokenStream)>,
    fn_names: Vec<String>,
    use_directives: Vec<(PathBuf, String, Position)>,
}

impl Component {
    fn new(fn_name: Ident, fn_names: Vec<String>) -> Self {
        Self {
            fn_name,
            token_stream: TokenStream::new(),
            params: Vec::new(),
            fns: Vec::new(),
            fn_names,
            use_directives: Vec::new(),
        }
    }

    fn params_to_ts(&self) -> Result<TokenStream> {
        let mut args = Vec::new();

        for (param_name, param_type) in &self.params {
            let param_name = Ident::new(param_name, Span::call_site());
            let param_type = parse_str::<Type>(param_type)
                .map_err(|e| anyhow!("Invalid param type: {param_type}, {e}"))?;

            args.push(quote! { #param_name: #param_type});
        }

        Ok(quote! {#(#args),*})
    }

    fn param_names_to_ts(&self) -> TokenStream {
        let args = self
            .params
            .iter()
            .map(|param| Ident::new(&param.0, Span::call_site()));
        quote! {#(#args),*}
    }
}
