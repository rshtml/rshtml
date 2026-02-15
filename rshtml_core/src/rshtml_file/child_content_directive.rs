use crate::rshtml_file::{Input, template::rust_identifier};
use proc_macro2::TokenStream;
use quote::quote;
use winnow::{
    ModalResult, Parser,
    combinator::{alt, not, peek},
};

pub fn child_content_directive<'a, 'ctx>(input: &mut Input<'a, 'ctx>) -> ModalResult<TokenStream> {
    (
        "child_content",
        alt(("()".void(), peek(not(rust_identifier)).void())),
    )
        .map(|(_, _)| quote! {child_content(__out__)?;})
        .parse_next(input)
}
