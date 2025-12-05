mod component;
mod include_directive;
mod inner_text;
mod match_expr;
mod raw;
mod rust_block;
mod rust_expr;
mod rust_expr_paren;
mod rust_expr_simple;
mod section_block;
mod template;
mod text;
mod use_directive;

use crate::Node;
use crate::compiler::component::ComponentCompiler;
use crate::compiler::include_directive::IncludeDirectiveCompiler;
use crate::compiler::inner_text::InnerTextCompiler;
use crate::compiler::match_expr::MatchExprCompiler;
use crate::compiler::raw::RawCompiler;
use crate::compiler::rust_block::RustBlockCompiler;
use crate::compiler::rust_expr::RustExprCompiler;
use crate::compiler::rust_expr_paren::RustExprParenCompiler;
use crate::compiler::rust_expr_simple::RustExprSimpleCompiler;
use crate::compiler::section_block::SectionBlockCompiler;
use crate::compiler::template::TemplateCompiler;
use crate::compiler::text::TextCompiler;
use crate::compiler::use_directive::UseDirectiveCompiler;
use crate::position::Position;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

pub struct Compiler {
    components: HashMap<String, (Node, TokenStream)>,
    pub layout: Option<Node>,
    sections: HashMap<String, TokenStream>,
    pub section_body: Option<TokenStream>,
    pub text_size: usize,
    pub files: Vec<(String, Position)>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            components: HashMap::new(),
            layout: None,
            sections: HashMap::new(),
            section_body: None,
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
            Node::PropsDirective(_, _) => Ok(quote! {}),
            Node::IncludeDirective(path, template) => {
                IncludeDirectiveCompiler::compile(self, path, *template)
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
            Node::SectionBlock(name, content, position) => {
                SectionBlockCompiler::compile(self, name, content, position)
            }
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

    pub fn section_names(&self) -> TokenStream {
        let mut token_stream = TokenStream::new();
        self.sections
            .keys()
            .for_each(|x| token_stream.extend(quote! {#x,}));

        quote! {[#token_stream]}
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
