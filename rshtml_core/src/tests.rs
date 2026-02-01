use crate::{Compiler, context::Context, rshtml_file};
use proc_macro2::{Span, TokenStream};
use std::path::Path;
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

    assert_eq!(true, res.contains("compile_error!"));
}

#[test]
fn test_rshtml_file() {
    let paths = [
        Path::new("views/rshtml_macro.rs.html"),
        Path::new("views/home.rs.html"),
    ];

    let mut ctx = Context::default();
    ctx.struct_fields = vec!["user".to_owned()];

    let result = rshtml_file::compile(paths[0], ctx);

    assert!(
        result.is_ok(),
        "\nRsHtml file compile failed!\n{}",
        result.err().map(|e| e.to_string()).unwrap_or_default()
    );

    let (fn_sign, fn_body, include_str, ctx) = result.unwrap();

    println!("--- FUNCTION SIGNATURE ---\n{}\n", fn_sign);
    println!("--- FUNCTION BODY ---\n{}\n", fn_body);
    println!("--- INCLUDE STR ---\n{}\n", include_str);
    println!("--- CONTEXT (Text Size: {})\n", ctx.text_size);
    println!("--- CONTEXT (Template Params: {:?})", ctx.template_params);
}
