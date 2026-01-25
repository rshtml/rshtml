use crate::rshtml_macro::{Context, template_params::template_params, text::text};
use proc_macro2::TokenStream;
use quote::quote;
use winnow::{
    ModalResult, Parser,
    combinator::{alt, eof, not, opt, peek, repeat},
};

pub fn template<'a>(input: &mut &'a str, ctx: &Context) -> ModalResult<TokenStream> {
    (
        // BOM?
        opt("\u{FEFF}"),
        // Template params veya değil
        alt((
            // &("@" ~ "(") ~ template_params
            peek(("@", "("))
                .and_then(|_| |i: &mut &'a str| template_params(i, ctx))
                .map(Some),
            // !("@" ~ "(")
            not(("@", "(")).map(|_| None),
        )),
        // template_content
        |i: &mut &'a str| template_content(i, ctx),
        // EOI
        eof,
    )
        .map(|(_, params_opt, content, _)| {
            // TokenStream oluştur
            match params_opt {
                Some(params) => {
                    // Params varsa
                    let param_tokens: Vec<_> = params
                        .iter()
                        .map(|(name, ty_opt)| {
                            let name_ident = syn::parse_str::<syn::Ident>(name).unwrap();
                            if let Some(ty_str) = ty_opt {
                                let ty = syn::parse_str::<syn::Type>(ty_str).unwrap();
                                quote! { #name_ident: #ty }
                            } else {
                                quote! { #name_ident }
                            }
                        })
                        .collect();

                    quote! {
                        |#(#param_tokens),*| {
                            #content
                        }
                    }
                }
                None => {
                    // Params yoksa
                    quote! {
                        #content
                    }
                }
            }
        })
        .parse_next(input)
}

pub fn template_content<'a>(input: &mut &'a str, ctx: &Context) -> ModalResult<TokenStream> {
    repeat(0.., |i: &mut &'a str| text(i, ctx))
        .fold(TokenStream::new, |mut acc, txt: String| {
            acc.extend(quote! { #txt });
            acc
        })
        .parse_next(input)
}
