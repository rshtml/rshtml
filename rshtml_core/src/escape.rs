use proc_macro2::TokenStream;
use quote::quote;

// pub fn escape<T: Display>(input: T, __f__: &mut dyn std::fmt::Write) -> std::fmt::Result {
//     for c in input.to_string().chars() {
//         match c {
//             '&' => write!(__f__, "{}", "&amp;")?,
//             '<' => write!(__f__, "{}", "&lt;")?,
//             '>' => write!(__f__, "{}", "&gt;")?,
//             '"' => write!(__f__, "{}", "&quot;")?,
//             '\'' => write!(__f__, "{}", "&#39;")?,
//             '/' => write!(__f__, "{}", "&#x2F;")?,
//             _ => write!(__f__, "{}", c)?,
//         }
//     }
//
//     Ok(())
// }

pub fn escape(input: TokenStream) -> TokenStream {
    quote! {
        for c in #input.to_string().chars() {
            match c {
                '&' => write!(__f__, "{}", "&amp;")?,
                '<' => write!(__f__, "{}", "&lt;")?,
                '>' => write!(__f__, "{}", "&gt;")?,
                '"' => write!(__f__, "{}", "&quot;")?,
                '\'' => write!(__f__, "{}", "&#39;")?,
                '/' => write!(__f__, "{}", "&#x2F;")?,
                _ => write!(__f__, "{}", c)?,
            }
        }
    }
}
