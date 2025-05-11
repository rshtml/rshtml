use crate::compiler::Compiler;
use proc_macro2::TokenStream;
use quote::quote;
use crate::Node;

pub struct SectionBlockCompiler;

impl SectionBlockCompiler {
    pub fn compile(compiler: &mut Compiler, name: &String, content: &Vec<Node>) -> TokenStream {
        let mut token_stream = TokenStream::new();

        for node in content {
            let ts = compiler.compile(node);
            token_stream.extend(quote! {#ts;});
        }

        compiler.sections.insert(name.clone(), token_stream);

        quote! {}
    }
}
