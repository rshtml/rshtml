use crate::diagnostic::Diagnostic;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use std::{collections::HashMap, env, fs, path::PathBuf};
use syn::{LitStr, parse_str, spanned::Spanned};
use winnow::{
    ModalResult, Parser,
    ascii::multispace0,
    combinator::{alt, cut_err, eof, fail, not, repeat},
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    stream::Stream,
    token::{any, none_of, take_while},
};

struct Context {
    // path: PathBuf,
    // source: String,
    diagnostic: Diagnostic,
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

    let ctx = Context {
        // path: full_path.to_owned(),
        // source: input.to_owned(),
        diagnostic: Diagnostic::new(HashMap::from([(full_path.to_owned(), input.to_owned())])),
    };

    let (text_size, expr_defs, body) = match template(&mut tokens, &ctx).and_then(|res| {
        eof.context(StrContext::Label("end of file"))
            .parse_next(&mut tokens)
            .map(|_| res)
    }) {
        Ok(res) => res,
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

fn template(input: &mut &str, ctx: &Context) -> ModalResult<(usize, TokenStream, TokenStream)> {
    repeat(
        1..,
        alt((
            (move |i: &mut &str| -> ModalResult<(TokenStream, TokenStream)> { expr(i, ctx) })
                .map(|(expr_def, expr)| (0, expr_def, expr))
                .context(StrContext::Label("rust block")),
            text.map(|t| (t.0, TokenStream::new(), t.1))
                .context(StrContext::Label("text")),
            fail.context(StrContext::Label("text or rust block")),
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

fn text(input: &mut &str) -> ModalResult<(usize, TokenStream)> {
    enum Chunk<'a> {
        Str(&'a str),
        Char(char),
    }

    repeat(
        1..,
        alt((
            take_while(1.., |c| c != '@').map(Chunk::Str),
            (not(("@", multispace0, "{")), any).map(|(_, c)| Chunk::Char(c)),
        )),
    )
    .fold(String::new, |mut acc, chunk| {
        match chunk {
            Chunk::Str(s) => acc.push_str(s),
            Chunk::Char(c) => acc.push(c),
        }
        acc
    })
    .map(|text| (text.chars().count(), quote! { write!(out, "{}", #text)?; }))
    .parse_next(input)
}

fn expr(input: &mut &str, ctx: &Context) -> ModalResult<(TokenStream, TokenStream)> {
    let start = *input;
    let checkpoint = input.checkpoint();

    ("@", multispace0, block).parse_next(input)?;

    let len = start.len() - input.len();
    let rust_block = &start[1..len];

    let def_ident = format_ident!("_exp{}", input.len());

    let output = match parse_str::<syn::Block>(rust_block) {
        Ok(block) => (
            quote! { let #def_ident = #block; _text_size += ::rshtml::TextSize(&#def_ident).text_size(); },
            quote! { ::rshtml::Exp(&(#def_ident)).render(out)?; },
        ),
        Err(e) => {
            let span = e.span();
            let start = span.start();
            let end = span.end();

            // ctx.diagnostic.caution(, position, title, lines, info, name_len)

            let tokens: TokenStream = rust_block.parse().map_err(|_| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &checkpoint,
                    StrContext::Label("Lex Error"),
                ))
            })?;

            (
                quote! { let #def_ident = #tokens; _text_size += ::rshtml::TextSize(&#def_ident).text_size(); },
                quote! { ::rshtml::Exp(&(#def_ident)).render(out)?; },
            )
        }
    };

    Ok(output)
}

fn block(input: &mut &str) -> ModalResult<()> {
    (
        "{",
        block_content,
        cut_err("}").context(StrContext::Expected(StrContextValue::CharLiteral('}'))),
    )
        .void()
        .parse_next(input)
}

fn block_content(input: &mut &str) -> ModalResult<()> {
    repeat(
        0..,
        alt((
            block.void(),
            line_comment,
            block_comment,
            string_literal,
            char_literal,
            take_while(1.., |c| !r#"{}"'/"#.contains(c)).void(),
            none_of('}').void(),
        )),
    )
    .parse_next(input)
}

fn line_comment(input: &mut &str) -> ModalResult<()> {
    ("//", take_while(0.., |c| c != '\n'))
        .void()
        .parse_next(input)
}

fn block_comment(input: &mut &str) -> ModalResult<()> {
    (
        "/*",
        repeat(
            0..,
            alt((
                block_comment,
                take_while(1.., |c| c != '/' && c != '*').void(),
                (not(alt(("/*", "*/"))), any).void(),
            )),
        )
        .fold(|| (), |_, _| ()),
        "*/",
    )
        .void()
        .parse_next(input)
}

fn string_literal(input: &mut &str) -> ModalResult<()> {
    (
        '"',
        repeat(0.., alt((("\\", any).void(), none_of(['"', '\\']).void()))).map(|_: Vec<()>| ()),
        '"',
    )
        .void()
        .parse_next(input)
}

fn char_literal(input: &mut &str) -> ModalResult<()> {
    (
        '\'',
        alt((("\\", any).void(), none_of(['"', '\\']).void())),
        '\'',
    )
        .void()
        .parse_next(input)
}
