use std::fs;
use std::io;
use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::quote;

pub fn create(struct_name: &str, generated_code: &str) -> io::Result<TokenStream> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let mut target_path = PathBuf::from(&manifest_dir);
    target_path.push("target");
    target_path.push("rshtml");

    fs::create_dir_all(&target_path)?;

    let file_name = format!("{}.rs", struct_name);
    target_path.push(&file_name);

    fs::write(&target_path, generated_code)?;

    let _ = std::process::Command::new("rustfmt")
        .arg("--config")
        .arg("max_width=10000")
        .arg(&target_path)
        .status();

    let full_path_str = target_path.to_str().ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "The generated file path is not valid UTF-8",
    ))?;

    Ok(quote! {include!(#full_path_str);})
}
