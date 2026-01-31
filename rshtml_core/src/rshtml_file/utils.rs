use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::{path::Path, str::FromStr};
use syn::{Ident, Member, parse_str};
pub fn extract_component_name(path: &Path) -> Option<String> {
    let filename = path.file_name().and_then(|n| n.to_str())?;
    let component_name = filename.strip_suffix(".rs.html").unwrap_or(filename);
    Some(component_name.to_owned())
}

pub fn escape_or_raw(expr_ts: TokenStream, is_escaped: bool, message: &str) -> TokenStream {
    if is_escaped {
        quote! { ::rshtml::Expr(&(#expr_ts)).render(&mut ::rshtml::EscapingWriter { inner: __f__ }, #message)?; }
    } else {
        quote! { ::rshtml::Expr(&(#expr_ts)).render(__f__, #message)?; }
    }
}

pub fn generate_fn_name(path: &Path) -> String {
    let component_name = extract_component_name(path).unwrap_or_default();
    let path_bytes = path.as_os_str().as_encoded_bytes();

    let mut hash: u64 = 5381;
    for c in path_bytes {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(*c as u64);
    }

    format!("{}_{:x}", component_name, hash)
}

pub fn params_to_ts(params: &mut [(&str, &str)]) -> TokenStream {
    params.sort_by(|a, b| a.0.cmp(b.0));

    let args = params.iter().map(|(param_name, param_type)| {
        let param_name = Ident::new(param_name, Span::call_site());
        let param_type = TokenStream::from_str(param_type).unwrap();
        quote! { #param_name: #param_type }
    });
    quote! {#(#args),*}
}

pub fn param_names_to_ts<S>(param_names: &mut [S]) -> TokenStream
where
    S: AsRef<str> + Clone,
{
    param_names.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));

    let args = param_names
        .as_ref()
        .iter()
        .map(|param| Ident::new(param.as_ref(), Span::call_site()));
    quote! {#(#args),*}
}

pub fn get_struct_field(expr: &str) -> Option<String> {
    let rest = expr
        .trim()
        .trim_start_matches('&')
        .trim()
        .strip_prefix("self.")?;

    let candidate = rest.split('.').next().unwrap_or(rest);

    if parse_str::<Member>(candidate).is_ok() {
        Some(candidate.to_string())
    } else {
        None
    }
}
