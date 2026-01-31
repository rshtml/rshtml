use crate::rshtml_file::Input;
use proc_macro2::TokenStream;
use quote::quote;
use winnow::{ModalResult, Parser, ascii::multispace0};

pub fn break_directive<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    ("break", multispace0)
        .map(|(_, _)| quote! { break })
        .parse_next(input)
}
