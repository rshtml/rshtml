use proc_macro2::TokenStream;
use quote::quote;
use rshtml_core::process_template;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use syn::__private::Span;
use tempfile::tempdir;

fn prepare(struct_name: &str, template_path: &str, fields: TokenStream, values: TokenStream, functions: TokenStream) -> std::io::Result<()> {
    let struct_name_ts = TokenStream::from_str(struct_name).unwrap();
    let ident = syn::Ident::new(struct_name, Span::call_site());
    let ts = process_template(template_path.to_string(), &ident);

    let test_code_str = quote! {
        pub use rshtml_core::traits::*;

        mod rshtml {
            pub use rshtml_core::functions::*;
            pub use rshtml_core::traits;
        }

        struct #struct_name_ts {
            #fields
        }

        impl #struct_name_ts {
            #functions
        }

        #ts

        fn main() {
            let mut page = #struct_name_ts {
                #values
            };

            println!("{}", page.render());
        }
    }
    .to_string();

    let dir = tempdir()?;
    let file_path = dir.path().join("test.rs");
    let mut file = File::create(&file_path)?;
    writeln!(file, "{}", test_code_str)?;

    let t = trybuild::TestCases::new();
    t.pass(&file_path);

    Ok(())
}

#[test]
pub fn test_empty() -> std::io::Result<()> {
    prepare("EmptyPage", "empty.rs.html", quote! {}, quote! {}, quote! {})
}

#[test]
pub fn test_if_else() -> std::io::Result<()> {
    prepare(
        "IfElsePage",
        "if_else.rs.html",
        quote! {
            is_ok: bool,
            count: i32,
        },
        quote! {
            is_ok: true,
            count: 10,
        },
        quote! {},
    )
}

#[test]
pub fn test_for() -> std::io::Result<()> {
    prepare(
        "ForPage",
        "for.rs.html",
        quote! {
            users: Vec<String>,
        },
        quote! {
            users: vec!["Alice".to_string(), "Bob".to_string()],
        },
        quote! {},
    )
}

#[test]
pub fn test_while() -> std::io::Result<()> {
    prepare(
        "WhilePage",
        "while.rs.html",
        quote! {
            count: i32,
        },
        quote! {
            count: 1,
        },
        quote! {
            fn increment(&mut self) -> String {
                self.count += 1;
                "".to_string()
            }
        },
    )
}

#[test]
pub fn test_match() -> std::io::Result<()> {
    prepare(
        "MatchPage",
        "match.rs.html",
        quote! {
            value: i32,
            data: Option<String>,
        },
        quote! {
            value: 10,
            data: Some("Hello".to_string()),
        },
        quote! {},
    )
}

#[test]
pub fn test_comment() -> std::io::Result<()> {
    prepare("CommentPage", "comment.rs.html", quote! {}, quote! {}, quote! {})
}

#[test]
fn test_test() {
    struct MyData {}

    impl MyData {
        fn to_string(&self) -> String {
            todo!()
        }
    }

    let x = MyData {};
    println!("{}", x.to_string());
}
