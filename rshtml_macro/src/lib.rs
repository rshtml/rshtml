#![doc(hidden)]

use proc_macro::TokenStream;
use rshtml_core::process_template;
use syn::{DeriveInput, Expr, Lit, Meta, parse_macro_input};

#[proc_macro_derive(RsHtml, attributes(rshtml))]
pub fn rshtml_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let template_name = match parse_template_path_from_attrs(&input.attrs) {
        Ok(Some(path)) => path,
        Ok(None) => {
            let struct_name_str = struct_name.to_string();
            let template_file = if let Some(stripped) = struct_name_str.strip_suffix("Page") {
                format!("{stripped}.rs.html")
            } else {
                format!("{struct_name_str}.rs.html")
            };

            template_file.to_lowercase()
        }
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    TokenStream::from(process_template(template_name, struct_name))
}

fn parse_template_path_from_attrs(attrs: &[syn::Attribute]) -> syn::Result<Option<String>> {
    for attr in attrs {
        if attr.path().is_ident("rshtml") {
            return match attr.parse_args::<Meta>() {
                Ok(Meta::NameValue(name_value)) => {
                    if name_value.path.is_ident("path") {
                        if let Expr::Lit(ref expr_lit) = name_value.value {
                            if let Lit::Str(lit_str) = &expr_lit.lit {
                                return Ok(Some(lit_str.value()));
                            }
                        }

                        Err(syn::Error::new_spanned(
                            name_value.value,
                            "Expected a string literal for the `path` argument, e.g., path = \"...\"",
                        ))
                    } else {
                        Err(syn::Error::new_spanned(
                            name_value.path,
                            "Expected argument name `path`, e.g., path = \"...\"",
                        ))
                    }
                }
                Ok(_) => Err(syn::Error::new_spanned(
                    attr,
                    "Expected `path = \"...\"` inside #[rshtml(...)]",
                )),
                Err(parse_err) => Err(parse_err),
            };
        }
    }

    Ok(None)
}
