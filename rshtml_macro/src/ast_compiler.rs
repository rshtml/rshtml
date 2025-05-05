#![allow(unused_variables, unused_imports)]

mod render_directive;

use crate::ast_compiler::render_directive::RenderDirectiveCompiler;
use proc_macro2::TokenStream;
use quote::quote;
use rshtml::Node;
use std::str::FromStr;

pub fn parse_and_compile_ast(template_path: &str) -> proc_macro2::TokenStream {
    let node = rshtml::parse(template_path);
    compile_ast(&node)
}

fn compile_ast(node: &Node) -> proc_macro2::TokenStream {
    let mut token_stream = proc_macro2::TokenStream::new();
    match node {
        Node::Template(nodes) => {
            for node in nodes {
                let ts = compile_ast(node);
                if let Node::RustBlock(x) = node {
                    dbg!("buradadaaaa");
                    dbg!(ts.clone());
                }
                token_stream.extend(quote! {#ts});
            }
            token_stream
        }
        Node::Text(text) => quote! { write!(f, "{}", #text)?; },
        Node::InnerText(inner_text) => quote! { write!(f, "{}", #inner_text)?; },
        Node::Comment(comment) => quote! { write!(f, "{}", #comment)?; },
        Node::ExtendsDirective(path) => quote! {},
        Node::RenderDirective(name) => RenderDirectiveCompiler::compile(&name),
        Node::RustBlock(contents) => quote! {},
        Node::RustExprSimple(expr) => quote! {},
        Node::RustExprParen(expr) => quote! {},
        Node::MatchExpr(name, arms) => quote! {},
        Node::RustExpr(exprs) => {
            let mut ts = proc_macro2::TokenStream::new();

            for (expr, inner_nodes) in exprs {
                let mut inner_ts = proc_macro2::TokenStream::new();
                for inner_node in inner_nodes {
                    let its = compile_ast(inner_node);
                    inner_ts.extend(quote! {#its;});
                }

                let expr_code = TokenStream::from_str(expr).unwrap();

                ts.extend(quote! { #expr_code { #inner_ts; } });
            }

            ts
        }
        Node::SectionDirective(name, content) => quote! {},
        Node::SectionBlock(name, content) => quote! {},
        Node::RenderBody => quote! {},
        Node::Component(name, parameters, body) => quote! {},
        Node::ChildContent => quote! {},
        Node::Raw(body) => quote! {},
        Node::UseDirective(name, path) => quote! {},
    }
}
