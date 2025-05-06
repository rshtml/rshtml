#![allow(unused_variables, unused_imports)]

mod match_expr;
mod render_directive;
mod rust_expr;
mod rust_expr_simple;

use crate::ast_compiler::match_expr::MatchExprCompiler;
use crate::ast_compiler::render_directive::RenderDirectiveCompiler;
use crate::ast_compiler::rust_expr::RustExprCompiler;
use crate::ast_compiler::rust_expr_simple::RustExprSimpleCompiler;
use proc_macro2::TokenStream;
use quote::quote;
use rshtml::Node;
use std::str::FromStr;

pub fn parse_and_compile_ast(template_path: &str) -> TokenStream {
    let node = rshtml::parse(template_path);
    compile_ast(&node)
}

fn compile_ast(node: &Node) -> TokenStream {
    let mut token_stream = TokenStream::new();
    match node {
        Node::Template(nodes) => {
            for node in nodes {
                let ts = compile_ast(node);
                token_stream.extend(quote! {#ts;});
            }
            token_stream
        }
        Node::Text(text) => quote! { write!(f, "{}", #text)? },
        Node::InnerText(inner_text) => quote! { write!(f, "{}", #inner_text)? },
        Node::Comment(comment) => quote! { write!(f, "{}", #comment)? },
        Node::ExtendsDirective(path) => quote! {},
        Node::RenderDirective(name) => RenderDirectiveCompiler::compile(&name),
        Node::RustBlock(contents) => quote! {},
        Node::RustExprSimple(expr) => RustExprSimpleCompiler::compile(expr),
        Node::RustExprParen(expr) => quote! {},
        Node::MatchExpr(name, arms) => MatchExprCompiler::compile(name, arms),
        Node::RustExpr(exprs) => RustExprCompiler::compile(exprs),
        Node::SectionDirective(name, content) => quote! {},
        Node::SectionBlock(name, content) => quote! {},
        Node::RenderBody => quote! {},
        Node::Component(name, parameters, body) => quote! {},
        Node::ChildContent => quote! {},
        Node::Raw(body) => quote! { write!(f, "{}", #body)? },
        Node::UseDirective(name, path) => quote! {},
    }
}
