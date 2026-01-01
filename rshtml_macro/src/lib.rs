#![doc(hidden)]

use proc_macro::TokenStream;
use rshtml_core::{process_function_like, process_template};
use syn::{Data, DeriveInput, Fields, LitStr, parse_macro_input};

#[proc_macro_derive(RsHtml, attributes(rshtml))]
pub fn rshtml_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let struct_generics = &input.generics;
    let struct_fields = get_struct_fields(&input.data);

    let (template_name, no_warn) = match parse_template_path_from_attrs(&input.attrs) {
        Ok(rshtml_config) => {
            let template_name = if let Some(path) = rshtml_config.path {
                path
            } else {
                let struct_name_str = struct_name.to_string();
                let mut template_file = if let Some(stripped) = struct_name_str.strip_suffix("Page")
                {
                    format!("{stripped}.rs.html")
                } else {
                    format!("{struct_name_str}.rs.html")
                };

                // template_file.to_lowercase()
                template_file = to_snake_case(&template_file);
                template_file
            };

            (template_name, rshtml_config.no_warn)
        }
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    TokenStream::from(process_template(
        template_name,
        struct_name,
        struct_generics,
        struct_fields,
        no_warn,
    ))
}

struct RsHtmlConfig {
    pub path: Option<String>,
    pub no_warn: bool,
}

fn parse_template_path_from_attrs(attrs: &[syn::Attribute]) -> syn::Result<RsHtmlConfig> {
    let mut config = RsHtmlConfig {
        path: None,
        no_warn: false,
    };

    for attr in attrs {
        if attr.path().is_ident("rshtml") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("path") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;
                    config.path = Some(s.value());
                    return Ok(());
                }

                if meta.path.is_ident("no_warn") {
                    config.no_warn = true;
                    return Ok(());
                }

                Err(meta.error("unsupported rshtml property"))
            })?;
        }
    }

    Ok(config)
}

fn to_snake_case(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let chars = s.chars().peekable();

    for c in chars {
        if c.is_uppercase() {
            if !result.is_empty() {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}

fn get_struct_fields(data: &Data) -> Vec<String> {
    match data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => fields_named
                .named
                .iter()
                .filter_map(|f| f.ident.as_ref().map(|id| id.to_string()))
                .collect(),
            Fields::Unnamed(fields_unnamed) => fields_unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, _field)| index.to_string())
                .collect(),
            _ => Vec::new(),
        },
        _ => Vec::new(),
    }
}

#[proc_macro]
pub fn v(input: TokenStream) -> TokenStream {
    TokenStream::from(process_function_like(input.into()))
}
