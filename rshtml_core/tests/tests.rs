use proc_macro2::TokenStream;
use quote::quote;
use rshtml_core::process_template;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use syn::__private::Span;
use tempfile::tempdir;

fn prepare(struct_name: &str, template_path: &str, fields: TokenStream, values: TokenStream) -> std::io::Result<()> {
    let struct_name_ts = TokenStream::from_str(struct_name).unwrap();
    let ident = syn::Ident::new(struct_name, Span::call_site());
    let ts = process_template(template_path.to_string(), &ident);

    let test_code_str = quote! {
        use rshtml_core::functions as rshtml;

        struct #struct_name_ts {
            #fields
        }

        #ts

        fn main() {
            let page = #struct_name_ts {
                #values
            };

            println!("{}", page.to_string());
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
    prepare("EmptyPage", "empty.rs.html", quote! {}, quote! {})
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
        }
    )
}

#[test]
pub fn test_process() -> std::io::Result<()> {
    let ident = syn::Ident::new("HomePage", Span::call_site());
    let ts = process_template("home.rs.html".to_string(), &ident);

    let test_code_str = quote! {
        #![allow(unused_variables, unused_imports, unused_mut, dead_code)]

        use rshtml_core::functions as rshtml;

        struct HomePage {
            title: String,
            content: String,
            card_count: usize,
            my_var: String,
            users: Vec<String>,
            abc: String,
            def: String,
            inner: String,
            hey: String,
            is_ok: bool,
        }

        #ts

        fn main() {
            let home_page = HomePage {
                title: "Home Page".to_string(),
                content: "This is the home page".to_string(),
                card_count: 10,
                my_var: "Hello".to_string(),
                users: vec!["Alice".to_string(), "Bob".to_string()],
                abc: "123".to_string(),
                def: "456".to_string(),
                inner: "789".to_string(),
                hey: "012".to_string(),
                is_ok: true,
            };

            println!("{}", home_page.to_string());
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