use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use std::{env, fs, path::PathBuf};
use syn::{LitStr, parse_str, spanned::Spanned};
use winnow::{
    Parser, Result,
    combinator::{alt, delimited, eof, repeat, terminated},
    error::StrContext,
    token::{any, none_of, take_until, take_while},
};

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

    let (text_size, expr_defs, body) = match terminated(template, eof).parse_next(&mut tokens) {
        Ok(res) => res,
        Err(err) => {
            let span = Span::call_site();
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
                0,
                TokenStream::new(),
                quote::quote_spanned! { span =>
                    compile_error!(#msg_lit);
                },
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

fn template(input: &mut &str) -> Result<(usize, TokenStream, TokenStream)> {
    repeat(
        0..,
        alt((
            expr.map(|(expr_def, expr)| (0, expr_def, expr)),
            text.map(|t| (t.0, TokenStream::new(), t.1)),
        )),
    )
    .fold(
        || (0, TokenStream::new(), TokenStream::new()),
        |(mut total_size, mut expr_defs, mut bodies), (text_size, expr_def, body)| {
            expr_defs.extend(expr_def);
            bodies.extend(body);
            total_size += text_size;
            (total_size, expr_defs, bodies)
        },
    )
    .parse_next(input)
}

fn text(input: &mut &str) -> Result<(usize, TokenStream)> {
    take_until(1.., "@{")
        .map(|text: &str| (text.chars().count(), quote! { out.write_str(#text)?; }))
        .parse_next(input)
}

fn expr(input: &mut &str) -> Result<(TokenStream, TokenStream)> {
    let rust_block = ("@", block)
        .map(|(_, rust_block)| rust_block)
        .parse_next(input)?;

    let def_ident = format_ident!("_exp{}", input.len());

    let output = if let Ok(block) = parse_str::<syn::Block>(rust_block) {
        (
            quote! { let #def_ident = #block; _text_size += ::rshtml::TextSize(&#def_ident).text_size(); },
            quote! { ::rshtml::Exp(&(#def_ident)).render(out)?; },
        )
    } else {
        (
            quote! { let #def_ident = #rust_block; _text_size += ::rshtml::TextSize(&#def_ident).text_size(); },
            quote! { ::rshtml::Exp(&(#def_ident)).render(out)?; },
        )
    };

    Ok(output)
}

fn block<'a>(input: &mut &'a str) -> Result<&'a str> {
    let start = *input;
    delimited("{", block_content, "}").parse_next(input)?;

    let len = start.len() - input.len();
    Ok(&start[..len])
}

fn block_content(input: &mut &str) -> Result<()> {
    repeat(
        0..,
        alt((
            block.void(),
            line_comment,
            block_comment,
            raw_string_literal,
            string_literal,
            char_literal,
            take_while(1.., |c| !"{}\"'/r".contains(c)).void(),
            any.void(),
        )),
    )
    .parse_next(input)
}

fn line_comment(input: &mut &str) -> Result<()> {
    ("//", take_while(0.., |c| c != '\n'))
        .void()
        .parse_next(input)
}

fn block_comment(input: &mut &str) -> Result<()> {
    (
        "/*",
        repeat(
            0..,
            alt((block_comment, take_until(1.., "*/").void(), any.void())),
        )
        .map(|_: Vec<()>| ()),
        "*/",
    )
        .void()
        .parse_next(input)
}

fn raw_string_literal(input: &mut &str) -> Result<()> {
    alt((
        // r#"..."# formatı (basitleştirilmiş, sadece 1 hash için, gramerinizdeki gibi)
        ("r#\"", take_until(0.., "\"#"), "\"#"),
        // r"..." formatı
        ("r\"", take_until(0.., "\""), "\""),
    ))
    .void()
    .parse_next(input)
}

fn string_literal(input: &mut &str) -> Result<()> {
    (
        '"',
        repeat(0.., alt((("\\", any).void(), none_of(['"', '\\']).void()))).map(|_: Vec<()>| ()),
        '"',
    )
        .void()
        .parse_next(input)
}

fn char_literal(input: &mut &str) -> Result<()> {
    (
        '\'',
        alt((("\\", any).void(), none_of(['"', '\\']).void())),
        '\'',
    )
        .void()
        .parse_next(input)
}
