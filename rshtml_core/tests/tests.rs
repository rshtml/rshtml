use proc_macro2::TokenStream;
use quote::quote;
use rshtml_core::process_template;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use syn::__private::Span;
use tempfile::tempdir;

fn prepare(
    struct_name: &str,
    template_path: &str,
    fields: TokenStream,
    values: TokenStream,
    functions: TokenStream,
) -> std::io::Result<()> {
    let struct_name_ts = TokenStream::from_str(struct_name).unwrap();
    let ident = syn::Ident::new(struct_name, Span::call_site());
    let ts = process_template(template_path.to_string(), &ident);

    let test_code_str = quote! {
        pub use rshtml_core::traits::*;

        mod rshtml {
            pub use rshtml_core::functions;
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

            println!("{}", page.render().unwrap());
        }
    }
    .to_string();

    let dir = tempdir()?;
    let file_path = dir.path().join("test.rs");
    let mut file = File::create(&file_path)?;
    writeln!(file, "{test_code_str}")?;

    let t = trybuild::TestCases::new();
    t.pass(&file_path);

    Ok(())
}

#[test]
pub fn test_empty() -> std::io::Result<()> {
    prepare(
        "EmptyPage",
        "empty.rs.html",
        quote! {},
        quote! {},
        quote! {},
    )
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
    prepare(
        "CommentPage",
        "comment.rs.html",
        quote! {},
        quote! {},
        quote! {},
    )
}

#[test]
pub fn test_simple_expression() -> std::io::Result<()> {
    prepare(
        "SimpleExpressionPage",
        "simple_expression.rs.html",
        quote! {
            value: i32,
            data: Option<String>,
            for_escape: String,
        },
        quote! {
            value: 10,
            data: Some("Hello".to_string()),
            for_escape: "'<script/>'".to_string(),
        },
        quote! {
            fn my_func(&self) -> String {
                "my func".to_string()
            }
        },
    )
}

#[test]
pub fn test_parentheses_expression() -> std::io::Result<()> {
    prepare(
        "ParenthesesExpressionPage",
        "parentheses_expression.rs.html",
        quote! {
            value: i32,
            data: String,
        },
        quote! {
            value: 10,
            data: "Hello".to_string(),
        },
        quote! {},
    )
}

#[test]
pub fn test_code_block() -> std::io::Result<()> {
    prepare(
        "CodeBlockPage",
        "code_block.rs.html",
        quote! {
            value: i32,
            data: String,
        },
        quote! {
            value: 10,
            data: "Hello".to_string(),
        },
        quote! {
            fn my_func(&self) -> String {
                let mut hold = "Func".to_string();
                hold.push_str(self.data.clone().as_str());
                hold
            }
        },
    )
}

#[test]
pub fn test_include() -> std::io::Result<()> {
    prepare(
        "IncludePage",
        "include.rs.html",
        quote! {
            value: i32,
            data: String,
        },
        quote! {
            value: 10,
            data: "Hello".to_string(),
        },
        quote! {
            fn my_func(&self) -> String {
                let mut hold = "Func".to_string();
                hold.push_str(self.data.clone().as_str());
                hold
            }
        },
    )
}

#[test]
pub fn test_layout() -> std::io::Result<()> {
    prepare(
        "ExtendsPage",
        "extends.rs.html",
        quote! {
            value: i32,
            data: String,
            for_escape: String,
        },
        quote! {
            value: 10,
            data: "Hello".to_string(),
            for_escape: "'<script/>'".to_string(),
        },
        quote! {
            fn my_func(&self) -> String {
                let mut hold = "Func".to_string();
                hold.push_str(self.data.clone().as_str());
                hold
            }
        },
    )
}

#[test]
pub fn test_layout_2() -> std::io::Result<()> {
    prepare(
        "Extends2Page",
        "extends2.rs.html",
        quote! {
            value: i32,
            data: String,
        },
        quote! {
            value: 10,
            data: "Hello".to_string(),
        },
        quote! {
            fn my_func(&self) -> String {
                let mut hold = "Func".to_string();
                hold.push_str(self.data.clone().as_str());
                hold
            }
        },
    )
}

#[test]
pub fn test_raw_block() -> std::io::Result<()> {
    prepare(
        "RawBlockPage",
        "raw_block.rs.html",
        quote! {
            value: i32,
            data: String,
        },
        quote! {
            value: 10,
            data: "Hello".to_string(),
        },
        quote! {
            fn my_func(&self) -> String {
                let mut hold = "Func".to_string();
                hold.push_str(self.data.clone().as_str());
                hold
            }
        },
    )
}

#[test]
pub fn test_component() -> std::io::Result<()> {
    prepare(
        "ComponentPage",
        "component.rs.html",
        quote! {
            value: i32,
            title: String,
            data: String,
            for_escape: String,
        },
        quote! {
            value: 10,
            title: "Component".to_string(),
            data: "Hello".to_string(),
            for_escape: "'<script/>'".to_string(),
        },
        quote! {
            fn my_func(&self) -> String {
                let mut hold = "Func".to_string();
                hold.push_str(self.data.clone().as_str());
                hold
            }
        },
    )
}

#[test]
pub fn test_continue_break() -> std::io::Result<()> {
    prepare(
        "ContinueBreakPage",
        "continue_break.rs.html",
        quote! {
            value: i32,
            data: String,
            users: Vec<String>,
        },
        quote! {
            value: 10,
            data: "Hello".to_string(),
            users: vec!["Alice".to_string(), "Bob".to_string(), "John".to_string()],
        },
        quote! {
            fn my_func(&self) -> String {
                let mut hold = "Func".to_string();
                hold.push_str(self.data.clone().as_str());
                hold
            }
        },
    )
}

#[test]
pub fn test_no_layout_with_section() -> std::io::Result<()> {
    prepare(
        "NoLayoutWithSectionPage",
        "no_layout_with_section.rs.html",
        quote! {
            value: i32,
        },
        quote! {
            value: 10,
        },
        quote! {},
    )
}

#[test]
pub fn test_functions() -> std::io::Result<()> {
    prepare(
        "FunctionsPage",
        "functions.rs.html",
        quote! {
            value: i32,
            date: chrono::DateTime<chrono::Utc>,
            users: Vec<String>,
        },
        quote! {
            value: 10,
            date: chrono::Utc::now(),
            users: vec!["Alice".to_string(), "Bob".to_string(), "John".to_string()],
        },
        quote! {},
    )
}

#[test]
pub fn test_escaping() -> std::io::Result<()> {
    prepare(
        "EscapingPage",
        "escaping.rs.html",
        quote! {
            my_var: String,
        },
        quote! {
            my_var: "<p>This is <strong>bold</strong> text.</p>".to_string(),
        },
        quote! {},
    )
}
