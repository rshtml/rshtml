mod component;
mod inner_text;
mod match_expr;
mod props_directive;
mod raw;
mod rust_block;
mod rust_expr;
mod rust_expr_paren;
mod rust_expr_simple;
mod template;
mod text;
mod use_directive;

use crate::Node;
use crate::compiler::component::ComponentCompiler;
use crate::compiler::inner_text::InnerTextCompiler;
use crate::compiler::match_expr::MatchExprCompiler;
use crate::compiler::props_directive::PropsDirectiveCompiler;
use crate::compiler::raw::RawCompiler;
use crate::compiler::rust_block::RustBlockCompiler;
use crate::compiler::rust_expr::RustExprCompiler;
use crate::compiler::rust_expr_paren::RustExprParenCompiler;
use crate::compiler::rust_expr_simple::RustExprSimpleCompiler;
use crate::compiler::template::TemplateCompiler;
use crate::compiler::text::TextCompiler;
use crate::compiler::use_directive::UseDirectiveCompiler;
use crate::position::Position;
use anyhow::Result;
use anyhow::anyhow;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::path::PathBuf;
use syn::Ident;
use syn::Type;
use syn::parse_str;

pub struct Compiler {
    use_directives: Vec<(PathBuf, String)>,
    components: HashMap<String, Component>,
    pub text_size: usize,
    pub files: Vec<(String, Position)>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            use_directives: Vec::new(),
            components: HashMap::new(),
            text_size: 0,
            files: Vec::new(),
        }
    }

    pub fn compile(&mut self, node: Node) -> Result<TokenStream> {
        match node {
            Node::Template(file, nodes, position) => {
                TemplateCompiler::compile(self, file, nodes, position)
            }
            Node::Text(text) => TextCompiler::compile(self, text),
            Node::InnerText(inner_text) => InnerTextCompiler::compile(self, inner_text),
            Node::Comment(_) => Ok(quote! {}),
            Node::PropsDirective(props, position) => {
                PropsDirectiveCompiler::compile(self, props, position)
            }
            Node::RustBlock(content, position) => {
                RustBlockCompiler::compile(self, content, position)
            }
            Node::RustExprSimple(expr, is_escaped, position) => {
                RustExprSimpleCompiler::compile(self, expr, is_escaped, position)
            }
            Node::RustExprParen(expr, is_escaped, position) => {
                RustExprParenCompiler::compile(self, expr, is_escaped, position)
            }
            Node::MatchExpr(head, arms, position) => {
                MatchExprCompiler::compile(self, head, arms, position)
            }
            Node::RustExpr(exprs, position) => RustExprCompiler::compile(self, exprs, position),
            Node::Component(name, parameters, body, position) => {
                ComponentCompiler::compile(self, name, parameters, body, position)
            }
            Node::ChildContent => Ok(quote! {child_content(__f__)?;}),
            Node::Raw(body) => RawCompiler::compile(self, body),
            Node::UseDirective(name, path, component, position) => {
                UseDirectiveCompiler::compile(self, name, path, *component, position)
            }
            Node::ContinueDirective => Ok(quote! {continue;}),
            Node::BreakDirective => Ok(quote! {break;}),
        }
    }

    pub fn components(&self) -> TokenStream {
        let mut token_stream = TokenStream::new();
        self.components.values().for_each(|component_data| {
            token_stream.extend(component_data.token_stream.to_owned());
        });

        token_stream
    }

    fn escape_or_raw(&self, expr_ts: TokenStream, is_escaped: bool) -> TokenStream {
        if is_escaped {
            quote! {write!(rshtml::EscapingWriter { inner: __f__ }, "{}", &(#expr_ts))?;}
        } else {
            quote! {write!(__f__, "{}", #expr_ts)?;}
        }
    }

    fn with_info(
        &self,
        expr_ts: TokenStream,
        position: Position,
        infos: Option<(&str, &str, bool)>,
    ) -> TokenStream {
        if cfg!(debug_assertions) {
            let positions = self
                .files
                .iter()
                .skip(1)
                .map(|(_, pos)| pos)
                .chain(std::iter::once(&position));

            let mappings: Vec<String> = self
                .files
                .iter()
                .zip(positions)
                .map(|((file, _), pos)| pos.as_info(file))
                .collect();

            let mapping = mappings.join(" > ");

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

#[derive(Clone)]
struct Component {
    token_stream: TokenStream,
    props: Vec<(String, String)>,
}

impl Component {
    fn new() -> Self {
        Self {
            token_stream: TokenStream::new(),
            props: Vec::new(),
        }
    }

    fn props_to_ts(&self) -> Result<TokenStream> {
        let mut args = Vec::new();

        for (prop_name, prop_type) in &self.props {
            let prop_name = Ident::new(&prop_name, Span::call_site());
            let prop_type = parse_str::<Type>(&prop_type).map_err(|e| anyhow!(e))?;

            args.push(quote! { #prop_name: #prop_type});
        }

        Ok(quote! {#(#args),*})
    }

    fn prop_names_to_ts(&self) -> TokenStream {
        let args = self
            .props
            .iter()
            .map(|prop| Ident::new(&prop.0, Span::call_site()));
        quote! {#(#args),*}
    }
}
