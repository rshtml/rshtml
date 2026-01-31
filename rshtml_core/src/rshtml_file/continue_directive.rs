use crate::rshtml_file::Input;
use proc_macro2::TokenStream;
use quote::quote;
use winnow::{ModalResult, Parser, ascii::multispace0};

pub fn continue_directive<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    ("continue", multispace0)
        .map(|(_, _)| quote! { continue })
        .parse_next(input)
}
