use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr, Lit, Meta, parse_macro_input};

#[proc_macro_derive(RsHtml, attributes(rshtml))]
pub fn rshtml_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    // Parse the template path from the #[rshtml(path = "...")] attribute
    let template_path = match parse_template_path_from_attrs(&input.attrs) {
        Ok(Some(path)) => path,
        Ok(None) => {
            // If the path attribute is missing, return a compile error
            return syn::Error::new_spanned(
                struct_name, // Span the error on the struct name
                "Missing `path` attribute, e.g., #[rshtml(path = \"template.rs.html\")]",
            )
            .to_compile_error()
            .into();
        }
        Err(err) => {
            // If parsing the attribute failed, return the error
            return err.to_compile_error().into();
        }
    };

    // TODO: Read and parse the template file content using `template_path`
    // TODO: Generate the `impl std::fmt::Display` block based on the template and struct fields

    // Placeholder implementation: generates an empty Display impl
    let generated_code = quote! {
        impl ::std::fmt::Display for #struct_name {
             fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                 // TODO: Implement the actual rendering logic here
                 // For now, just write the template path for debugging
                 write!(f, "Template path: {}", #template_path)
             }
        }
    };

    TokenStream::from(generated_code)
}

fn parse_template_path_from_attrs(attrs: &[syn::Attribute]) -> syn::Result<Option<String>> {
    //unimplemented!("Attribute parsing not implemented yet");
    // Ok("placeholder/template.rs.html".to_string())

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

                        Err(syn::Error::new_spanned(name_value.value, "Expected a string literal for the `path` argument, e.g., path = \"...\""))
                    } else {
                        Err(syn::Error::new_spanned(name_value.path, "Expected argument name `path`, e.g., path = \"...\""))
                    }
                }
                Ok(_) => Err(syn::Error::new_spanned(attr, "Expected `path = \"...\"` inside #[rshtml(...)]")),
                Err(parse_err) => Err(parse_err),
            };
        }
    }

    Ok(None)
}
