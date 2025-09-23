use crate::position::Position;
use proc_macro2::TokenStream;
use quote::quote;
// use std::str::FromStr;

pub(crate) trait ToInfo {
    fn with_info(
        ts: TokenStream,
        files: &Vec<(String, Position)>,
        position: &Position,
    ) -> TokenStream;
}

impl ToInfo for TokenStream {
    fn with_info(
        ts: TokenStream,
        files: &Vec<(String, Position)>,
        position: &Position,
    ) -> TokenStream {
        // let mapping = TokenStream::from_str(&position.as_info(files)).unwrap();
        // quote! {i!(#ts => #mapping)}
        if cfg!(debug_assertions) {
            let positions = files
                .iter()
                .skip(1)
                .map(|(_, pos)| pos) // Sonraki elemanların pozisyonlarını al
                .chain(std::iter::once(position));

            let mappings: Vec<String> = files
                .iter()
                .zip(positions)
                .map(|((file, _), pos)| pos.as_info(&file))
                .collect();

            let mapping = mappings.join(" > ");
            quote! {{#mapping;#ts}}
        } else {
            ts
        }
    }
}
