use quote::quote;

pub struct RenderDirectiveCompiler;

impl RenderDirectiveCompiler {
    pub fn compile(name: &str) -> proc_macro2::TokenStream {
        quote! {}
    }
}