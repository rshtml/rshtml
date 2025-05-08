mod compiler;

use crate::compiler::parse_and_compile_ast;
use proc_macro::TokenStream;
use quote::quote;
use rshtml::config::Config;
use serde::Deserialize;
use std::path::Path;
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
                format!("{}.rs.html", stripped)
            } else {
                format!("{}.rs.html", struct_name_str)
            };

            template_file.to_lowercase()
        }
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let config = get_config_from_toml();
    let compiled_ast_tokens = parse_and_compile_ast(&template_name, config);

    //dbg!("DEBUG: Generated write_calls TokenStream:\n{:#?}", compiled_ast_tokens.to_string());

    let generated_code = quote! {
        impl ::std::fmt::Display for #struct_name {
             fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {

                #compiled_ast_tokens

                Ok(())
             }
        }
    };

    TokenStream::from(generated_code)
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

fn get_config_from_toml() -> Config {
    #[derive(Deserialize, Debug, Clone)]
    pub struct MetadataConfig {
        pub views_base_path: Option<String>,
        pub layout: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    struct Metadata {
        rshtml: Option<MetadataConfig>,
    }

    #[derive(Deserialize, Debug)]
    struct Package {
        metadata: Option<Metadata>,
    }

    #[derive(Deserialize, Debug)]
    struct Manifest {
        package: Option<Package>,
    }

    let mut config = &mut Config::default();

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let cargo_toml_path = Path::new(&manifest_dir).join("Cargo.toml");
        if let Ok(content) = std::fs::read_to_string(cargo_toml_path) {
            match toml::from_str::<Manifest>(&content) {
                Ok(manifest) => {
                    if let Some(pkg) = manifest.package {
                        if let Some(metadata) = pkg.metadata {
                            if let Some(toml_config) = metadata.rshtml {
                                if let Some(path_str) = toml_config.views_base_path {
                                    config = config.set_views_base_path(path_str);
                                }
                                if let Some(layout_str) = toml_config.layout {
                                    config.set_layout(layout_str);
                                }
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }

    config.clone()
}
