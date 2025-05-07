#![allow(unused_variables, unused_imports)]

mod match_expr;
mod render_directive;
mod rust_block;
mod rust_expr;
mod rust_expr_paren;
mod rust_expr_simple;
mod use_directive;

use crate::compiler::match_expr::MatchExprCompiler;
use crate::compiler::render_directive::RenderDirectiveCompiler;
use crate::compiler::rust_block::RustBlockCompiler;
use crate::compiler::rust_expr::RustExprCompiler;
use crate::compiler::rust_expr_paren::RustExprParenCompiler;
use crate::compiler::rust_expr_simple::RustExprSimpleCompiler;
use crate::compiler::use_directive::UseDirectiveCompiler;
use proc_macro2::TokenStream;
use quote::quote;
use rshtml::Node;
use rshtml::config::Config;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

pub fn parse_and_compile_ast(template_path: &str, config: Config) -> TokenStream {
    let node = rshtml::parse(template_path, config);
    Compiler::new().compile(&node)
}

struct Compiler {
    use_directives: Vec<(String, PathBuf)>,
    components: HashMap<String, Node>,
}

impl Compiler {
    fn new() -> Self {
        Compiler {
            use_directives: Vec::new(),
            components: HashMap::new(),
        }
    }

    fn compile(&mut self, node: &Node) -> TokenStream {
        let mut token_stream = TokenStream::new();
        match node {
            Node::Template(nodes) => {
                for node in nodes {
                    let ts = self.compile(node);
                    token_stream.extend(quote! {# ts;});
                }
                token_stream
            }
            Node::Text(text) => quote! { write ! (f, "{}", # text)? },
            Node::InnerText(inner_text) => quote! { write ! (f, "{}", # inner_text)? },
            Node::Comment(comment) => quote! {},
            Node::ExtendsDirective(path) => quote! {},
            Node::RenderDirective(name) => RenderDirectiveCompiler::compile(&name),
            Node::RustBlock(contents) => RustBlockCompiler::compile(self, contents),
            Node::RustExprSimple(expr) => RustExprSimpleCompiler::compile(expr),
            Node::RustExprParen(expr) => RustExprParenCompiler::compile(expr),
            Node::MatchExpr(name, arms) => MatchExprCompiler::compile(self, name, arms),
            Node::RustExpr(exprs) => RustExprCompiler::compile(self, exprs),
            Node::SectionDirective(name, content) => quote! {},
            Node::SectionBlock(name, content) => quote! {},
            Node::RenderBody => quote! {},
            Node::Component(name, parameters, body) => quote! {},
            Node::ChildContent => quote! {},
            Node::Raw(body) => quote! { write ! (f, "{}", # body)? },
            Node::UseDirective(name, path, component) => UseDirectiveCompiler::compile(self, name, path, component),
        }
    }
}
