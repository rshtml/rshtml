use crate::{Compiler, rshtml_file};
use proc_macro2::{Span, TokenStream};
use std::path::{Path, PathBuf};
use syn::{Generics, Ident};

#[test]
fn test_compiler() {
    let paths = [
        Path::new("views/rshtml_macro.rs.html"),
        Path::new("views/home.rs.html"),
    ];
    let ident = Ident::new("RsHtmlMacro", Span::call_site());

    let mut compiler = Compiler::new(ident, Generics::default(), vec!["user".to_owned()]);

    let result: TokenStream = compiler.compile(paths[0]);
    let res = result.to_string();
    println!("{res}");

    assert!(!res.contains("compile_error!"));
}

#[test]
fn test_rshtml_file() {
    let paths = [
        Path::new("views/rshtml_macro.rs.html"),
        Path::new("views/home.rs.html"),
    ];

    let base_dir = PathBuf::from("views");
    let struct_fields = vec!["user".to_owned()];

    let result = rshtml_file::compile(paths[0], &base_dir, &struct_fields);

    assert!(
        result.is_ok(),
        "\nRsHtml file compile failed!\n{}",
        result.err().map(|e| e.to_string()).unwrap_or_default()
    );

    let (fn_sign, fn_body, include_str, info, source) = result.unwrap();

    println!("--- FUNCTION SIGNATURE ---\n{}\n", fn_sign);
    println!("--- FUNCTION BODY ---\n{}\n", fn_body);
    println!("--- INCLUDE STR ---\n{}\n", include_str);
    println!("--- CONTEXT (Text Size: {})\n", info.text_size);
    println!("--- CONTEXT (Template Params: {:?})", info.template_params);
    println!("--- SOURCE LENGTH: {}", source.len());
}
