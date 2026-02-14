use proc_macro2::TokenStream;
use quote::quote;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub fn create(path: &Path, generated_code: &str) -> io::Result<TokenStream> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let mut target_path = PathBuf::from(&manifest_dir);
    target_path.push("target");
    target_path.push("rshtml");

    target_path.push(path);

    let parent = target_path.parent().ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "The file path is not valid",
    ))?;
    fs::create_dir_all(parent)?;

    fs::write(&target_path, generated_code)?;

    let mut include_path = PathBuf::new();
    include_path.push("target");
    include_path.push("rshtml");
    include_path.push(path);

    let include_path_str = target_path.to_str().ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "The generated file path is not valid UTF-8",
    ))?;

    Ok(quote! {include!(#include_path_str);})
}
