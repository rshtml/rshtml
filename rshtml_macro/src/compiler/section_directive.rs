use crate::compiler::Compiler;
use proc_macro2::TokenStream;
use quote::quote;
use rshtml::Node;
use rshtml::node::SectionDirectiveContent;
use std::str::FromStr;

pub struct SectionDirectiveCompiler;

impl SectionDirectiveCompiler {
    pub fn compile(compiler: &mut Compiler, name: &String, content: &SectionDirectiveContent) -> TokenStream {
        let content_ts = match content {
            SectionDirectiveContent::Text(text) => quote! { write!(f, "{}", #text)? },
            SectionDirectiveContent::RustExprSimple(expr) => compiler.compile(&Node::RustExprSimple(expr.clone())),
        };

        compiler.sections.insert(name.clone(), content_ts.clone());

        quote! {}
    }
}
