mod component;
mod extends_directive;
mod match_expr;
mod render_body;
mod render_directive;
mod rust_block;
mod rust_expr;
mod rust_expr_paren;
mod rust_expr_simple;
mod section_block;
mod section_directive;
mod use_directive;

use crate::Node;
use crate::compiler::component::ComponentCompiler;
use crate::compiler::extends_directive::ExtendsDirectiveCompiler;
use crate::compiler::match_expr::MatchExprCompiler;
use crate::compiler::render_body::RenderBodyCompiler;
use crate::compiler::render_directive::RenderDirectiveCompiler;
use crate::compiler::rust_block::RustBlockCompiler;
use crate::compiler::rust_expr::RustExprCompiler;
use crate::compiler::rust_expr_paren::RustExprParenCompiler;
use crate::compiler::rust_expr_simple::RustExprSimpleCompiler;
use crate::compiler::section_block::SectionBlockCompiler;
use crate::compiler::section_directive::SectionDirectiveCompiler;
use crate::compiler::use_directive::UseDirectiveCompiler;
use crate::config::Config;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

// TODO: in rust block, process the texts @: and <text></text>
// TODO: Manage from_str errors

pub struct Compiler {
    use_directives: Vec<(String, PathBuf)>,
    components: HashMap<String, Node>,
    layout_directive: PathBuf,
    pub layout: Option<Node>,
    sections: HashMap<String, TokenStream>,
    pub section_body: Option<TokenStream>,
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
        }
    }

    pub fn compile(&mut self, node: &Node) -> TokenStream {
        let mut token_stream = TokenStream::new();
        match node {
            Node::Template(nodes) => {
                for node in nodes {
                    let ts = self.compile(node);
                    token_stream.extend(quote! {#ts;});
                }
                token_stream
            }
            Node::Text(text) => quote! { write!(f, "{}", #text)? },
            Node::InnerText(inner_text) => quote! { write!(f, "{}", #inner_text)? },
            Node::Comment(comment) => quote! {},
            Node::ExtendsDirective(path, layout) => ExtendsDirectiveCompiler::compile(self, path, layout),
            Node::RenderDirective(name) => RenderDirectiveCompiler::compile(self, &name),
            Node::RustBlock(contents) => RustBlockCompiler::compile(self, contents),
            Node::RustExprSimple(expr) => RustExprSimpleCompiler::compile(expr),
            Node::RustExprParen(expr) => RustExprParenCompiler::compile(expr),
            Node::MatchExpr(name, arms) => MatchExprCompiler::compile(self, name, arms),
            Node::RustExpr(exprs) => RustExprCompiler::compile(self, exprs),
            Node::SectionDirective(name, content) => SectionDirectiveCompiler::compile(self, name, content),
            Node::SectionBlock(name, content) => SectionBlockCompiler::compile(self, name, content),
            Node::RenderBody => RenderBodyCompiler::compile(self),
            Node::Component(name, parameters, body) => ComponentCompiler::compile(self, name, parameters, body),
            Node::ChildContent => quote! {child_content(f)?;},
            Node::Raw(body) => quote! { write!(f, "{}", #body)? },
            Node::UseDirective(name, path, component) => UseDirectiveCompiler::compile(self, name, path, component),
        }
    }
}
