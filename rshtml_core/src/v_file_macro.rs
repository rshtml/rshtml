use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use std::{env, fs, path::PathBuf};
use syn::{LitStr, spanned::Spanned};
use winnow::{
    ModalResult, Parser,
    combinator::{eof, terminated},
    error::StrContext,
};

enum Node {
    Expr(TokenStream),
    Text(String),
}

pub fn compile(path: LitStr) -> TokenStream {
    let path = path.value();
    let span = path.span();

    let base_dir = match env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|_| env::current_dir())
    {
        Ok(base_dir) => base_dir,
        Err(e) => {
            let msg = format!("Failed to get current directory: {}", e);
            return quote_spanned! { span =>  compile_error!(#msg); };
        }
    };

    let full_path = base_dir.join(path);

    let input = match fs::read_to_string(&full_path) {
        Ok(content) => content,
        Err(e) => {
            let msg = format!("v_file! failed to read '{}': {}", full_path.display(), e);
            return quote_spanned! { span => compile_error!(#msg); };
        }
    };

    let mut tokens = input.as_str();

    let (expr_defs, body, text_size) = match terminated(template, eof).parse_next(&mut tokens) {
        Ok((expr_defs, nodes)) => {
            let mut body = TokenStream::new();
            let mut text_buffer = String::new();
            let mut first = true;
            let mut text_size = 0;

            for node in nodes {
                match node {
                    Node::Expr(tokens) => {
                        if !text_buffer.is_empty() {
                            text_buffer.push(' ');
                            body.extend(quote! { write!(out, "{}", #text_buffer)?; });
                            text_buffer.clear();
                        }

                        body.extend(tokens);
                    }
                    Node::Text(text) => {
                        if !first {
                            text_buffer.push(' ');
                        }
                        text_buffer.push_str(&text);
                        text_size += text.len();
                        first = false;
                    }
                }
            }

            if !text_buffer.is_empty() {
                body.extend(quote! { write!(out, "{}", #text_buffer)?; });
            }

            (expr_defs, body, text_size)
        }
        Err(e) => {
            let span = Span::call_site();
            let err = e.into_inner().unwrap();
            let msg = err
                .context()
                .filter_map(|c| match c {
                    StrContext::Label(l) => Some(l.to_string()),
                    StrContext::Expected(e) => Some(format!("expected {}", e)),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join(": ");

            let msg = format!("compile error: {msg}");
            let msg_lit = syn::LitStr::new(&msg, span);

            (
                TokenStream::new(),
                quote::quote_spanned! { span =>
                    compile_error!(#msg_lit);
                },
                0,
            )
        }
    };

    let full_path_str = full_path.to_string_lossy();

    quote! {
        ::rshtml::ViewFn::new({
            let _ = include_str!(#full_path_str);

            let mut _text_size = #text_size;
            #expr_defs

            (
                move |out: &mut dyn std::fmt::Write| -> std::fmt::Result {
                    #body
                    Ok(())
                },
                _text_size
            )
        })
    }
}

fn template(input: &mut &str) -> ModalResult<(TokenStream, Vec<Node>)> {
    todo!()
}

fn expr(input: &mut &str) -> ModalResult<(TokenStream, Node)> {
    todo!()
}

fn text(input: &mut &str) -> ModalResult<(TokenStream, Node)> {
    todo!()
}
