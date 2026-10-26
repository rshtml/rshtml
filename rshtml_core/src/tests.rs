use crate::{Compiler, rshtml_file};
use proc_macro2::{Span, TokenStream};
use std::path::{Path, PathBuf};
use syn::{Generics, Ident};

#[test]
fn test_compiler() {
    let paths = [Path::new("views/rshtml_macro.rs.html"), Path::new("views/home.rs.html")];
    let ident = Ident::new("RsHtmlMacro", Span::call_site());

    let mut compiler = Compiler::new(ident, Generics::default(), false);

    let result: TokenStream = compiler.compile(paths[0]);
    let res = result.to_string();
    println!("{res}");

    assert!(!res.contains("compile_error!"));
}

#[test]
fn test_rshtml_file() {
    let paths = [Path::new("views/rshtml_macro.rs.html"), Path::new("views/home.rs.html")];

    let base_dir = PathBuf::from("views");

    let parse_result = rshtml_file::parse(paths[0], &base_dir);

    assert!(
        parse_result.is_ok(),
        "\nRsHtml file parse failed!\n{}",
        parse_result.err().unwrap_or_default()
    );

    let (ops, info, source) = parse_result.unwrap();

    let codegen_result = rshtml_file::codegen(
        &info.full_path,
        &info.fn_name,
        &info.use_directives,
        ops,
        &source,
        true,
    );

    assert!(
        codegen_result.is_ok(),
        "\nRsHtml file codegen failed!\n{}",
        codegen_result.err().unwrap_or_default()
    );

    let (fn_sign, fn_body, include_str, _root_fn, text_size) = codegen_result.unwrap();

    println!("--- FUNCTION SIGNATURE ---\n{}\n", fn_sign);
    println!("--- FUNCTION BODY ---\n{}\n", fn_body);
    println!("--- INCLUDE STR ---\n{}\n", include_str);
    println!("--- CONTEXT (Text Size: {})\n", text_size);
    println!("--- SOURCE LENGTH: {}", source.len());
}
