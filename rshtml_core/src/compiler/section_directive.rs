use crate::Node;
use crate::compiler::Compiler;
use crate::node::SectionDirectiveContent;
use crate::position::Position;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

pub struct SectionDirectiveCompiler;

impl SectionDirectiveCompiler {
    pub fn compile(
        compiler: &mut Compiler,
        name: String,
        content: SectionDirectiveContent,
        position: Position,
    ) -> Result<TokenStream> {
        let content_ts = match content {
            SectionDirectiveContent::Text(text) => compiler.compile(Node::Text(text))?,
            SectionDirectiveContent::RustExprSimple(expr, is_escaped) => {
                compiler.compile(Node::RustExprSimple(expr, is_escaped, position))?
            }
        };

        compiler.sections.insert(name.to_owned(), content_ts);

        Ok(quote! {})
    }
}
