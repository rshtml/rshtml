mod component;
mod extends_directive;
mod inner_text;
mod match_expr;
mod raw;
mod render_body;
mod render_directive;
mod rust_block;
mod rust_expr;
mod rust_expr_paren;
mod rust_expr_simple;
mod section_block;
mod section_directive;
mod text;
mod use_directive;

use crate::compiler::component::ComponentCompiler;
use crate::compiler::extends_directive::ExtendsDirectiveCompiler;
use crate::compiler::inner_text::InnerTextCompiler;
use crate::compiler::match_expr::MatchExprCompiler;
use crate::compiler::raw::RawCompiler;
use crate::compiler::render_body::RenderBodyCompiler;
use crate::compiler::render_directive::RenderDirectiveCompiler;
use crate::compiler::rust_block::RustBlockCompiler;
use crate::compiler::rust_expr::RustExprCompiler;
use crate::compiler::rust_expr_paren::RustExprParenCompiler;
use crate::compiler::rust_expr_simple::RustExprSimpleCompiler;
use crate::compiler::section_block::SectionBlockCompiler;
use crate::compiler::section_directive::SectionDirectiveCompiler;
use crate::compiler::text::TextCompiler;
use crate::compiler::use_directive::UseDirectiveCompiler;
use crate::position::Position;
use crate::Node;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::path::PathBuf;
// TODO: Maybe use like syn::parse2::<Expr> for compiler control, and get error from parser

pub struct Compiler {
    use_directives: Vec<(String, PathBuf)>,
    components: HashMap<String, Node>,
    layout_directive: PathBuf,
    pub layout: Option<Node>,
    sections: HashMap<String, TokenStream>,
    pub section_body: Option<TokenStream>,
    pub text_size: usize,
    files: Vec<(String, Position)>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            use_directives: Vec::new(),
            components: HashMap::new(),
            layout_directive: PathBuf::new(),
            layout: None,
            sections: HashMap::new(),
            section_body: None,
            text_size: 0,
            files: Vec::new(),
        }
    }

    pub fn compile(&mut self, node: &Node) -> Result<TokenStream> {
        match node {
            Node::Template(file, nodes, position) => {
                if !file.is_empty() {
                    self.files.push((file.clone(), position.clone()));
                }

                let mut token_stream = TokenStream::new();
                for node in nodes {
                    let ts = self.compile(node)?;
                    token_stream.extend(quote! {#ts});
                }

                if !file.is_empty() {
                    self.files.pop();
                }

                Ok(token_stream)
            }
            Node::Text(text, position) => TextCompiler::compile(self, text),
            Node::InnerText(inner_text, position) => InnerTextCompiler::compile(self, inner_text),
            Node::Comment(_, _) => Ok(quote! {}),
            Node::ExtendsDirective(path, layout, position) => {
                ExtendsDirectiveCompiler::compile(self, path, layout)
            }
            Node::RenderDirective(name, position) => RenderDirectiveCompiler::compile(self, name),
            Node::RustBlock(content, position) => RustBlockCompiler::compile(self, content),
            Node::RustExprSimple(expr, is_escaped, position) => {
                RustExprSimpleCompiler::compile(self, expr, is_escaped, position)
            }
            Node::RustExprParen(expr, is_escaped, position) => {
                RustExprParenCompiler::compile(self, expr, is_escaped, position)
            }
            Node::MatchExpr((head, head_position), arms, position) => {
                MatchExprCompiler::compile(self, (head, head_position), arms, position)
            }
            Node::RustExpr(exprs, position) => RustExprCompiler::compile(self, exprs, position),
            Node::SectionDirective(name, content, position) => {
                SectionDirectiveCompiler::compile(self, name, content, position)
            }
            Node::SectionBlock((name, name_position), content, position) => {
                SectionBlockCompiler::compile(self, name, content, position)
            }
            Node::RenderBody(position) => RenderBodyCompiler::compile(self),
            Node::Component(name, parameters, body, position) => {
                ComponentCompiler::compile(self, name, parameters, body, position)
            }
            Node::ChildContent(position) => Ok(quote! {child_content(__f__)?;}),
            Node::Raw(body, position) => RawCompiler::compile(self, body),
            Node::UseDirective(name, path, component, position) => {
                UseDirectiveCompiler::compile(self, name, path, component, position)
            }
            Node::ContinueDirective(position) => Ok(quote! {continue;}),
            Node::BreakDirective(position) => Ok(quote! {break;}),
        }
    }

    pub fn section_names(&self) -> TokenStream {
        let mut token_stream = TokenStream::new();
        self.sections
            .keys()
            .for_each(|x| token_stream.extend(quote! {#x,}));

        quote! {[#token_stream]}
    }

    fn escape_or_raw(&self, expr_ts: TokenStream, is_escaped: &bool) -> TokenStream {
        if *is_escaped {
            quote! {write!(rshtml::EscapingWriter { inner: __f__ }, "{}", &(#expr_ts))?;}
        } else {
            quote! {write!(__f__, "{}", #expr_ts)?;}
        }
    }
}
