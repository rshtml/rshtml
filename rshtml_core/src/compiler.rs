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

use crate::Node;
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
        }
    }

    pub fn compile(&mut self, node: &Node) -> Result<TokenStream> {
        match node {
            Node::Template(nodes) => {
                let mut token_stream = TokenStream::new();
                for node in nodes {
                    let ts = self.compile(node)?;
                    token_stream.extend(quote! {#ts});
                }
                Ok(token_stream)
            }
            Node::Text(text) => TextCompiler::compile(self, text),
            Node::InnerText(inner_text) => InnerTextCompiler::compile(self, inner_text),
            Node::Comment(_) => Ok(quote! {}),
            Node::ExtendsDirective(path, layout) => {
                ExtendsDirectiveCompiler::compile(self, path, layout)
            }
            Node::RenderDirective(name) => RenderDirectiveCompiler::compile(self, name),
            Node::RustBlock(contents) => RustBlockCompiler::compile(self, contents),
            Node::RustExprSimple(expr, is_escaped) => {
                RustExprSimpleCompiler::compile(self, expr, is_escaped)
            }
            Node::RustExprParen(expr, is_escaped) => {
                RustExprParenCompiler::compile(self, expr, is_escaped)
            }
            Node::MatchExpr(name, arms) => MatchExprCompiler::compile(self, name, arms),
            Node::RustExpr(exprs) => RustExprCompiler::compile(self, exprs),
            Node::SectionDirective(name, content) => {
                SectionDirectiveCompiler::compile(self, name, content)
            }
            Node::SectionBlock(name, content) => SectionBlockCompiler::compile(self, name, content),
            Node::RenderBody => RenderBodyCompiler::compile(self),
            Node::Component(name, parameters, body) => {
                ComponentCompiler::compile(self, name, parameters, body)
            }
            Node::ChildContent => Ok(quote! {child_content(__f__)?;}),
            Node::Raw(body) => RawCompiler::compile(self, body),
            Node::UseDirective(name, path, component) => {
                UseDirectiveCompiler::compile(self, name, path, component)
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

    fn escape(&self, input: TokenStream) -> TokenStream {
        quote! {
            for c in #input.to_string().chars() {
                match c {
                    '&' => write!(__f__, "{}", "&amp;")?,
                    '<' => write!(__f__, "{}", "&lt;")?,
                    '>' => write!(__f__, "{}", "&gt;")?,
                    '"' => write!(__f__, "{}", "&quot;")?,
                    '\'' => write!(__f__, "{}", "&#39;")?,
                    '/' => write!(__f__, "{}", "&#x2F;")?,
                    _ => write!(__f__, "{}", c)?,
                }
            }
        }
    }

    fn escape_or_raw(&self, expr_ts: TokenStream, is_escaped: &bool) -> TokenStream {
        if *is_escaped {
            let escaped = self.escape(quote! {(#expr_ts)});
            quote! {#escaped}
        } else {
            quote! {write!(__f__, "{}", #expr_ts)?;}
        }
    }
}
