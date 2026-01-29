mod component;
mod extensions;
mod inner_text;
mod rust_block;
mod rust_stmt;
mod simple_expr;
mod simple_expr_paren;
mod template;
mod template_params;
mod text;
mod use_directive;

use crate::{
    diagnostic::Diagnostic,
    rshtml_macro::template::{generate_fn_name, param_names_to_ts},
};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use std::{
    collections::{HashMap, HashSet},
    env, fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};
use syn::{Ident, LitStr, spanned::Spanned};
use template::template;
use winnow::{
    Stateful,
    error::{StrContext, StrContextValue},
};

// TODO: Consider whether the import paths in the `use` statement should start from the location of the file.

pub type Input<'a> = Stateful<&'a str, &'a mut Context>;

#[derive(Debug, Default, Clone)]
pub struct Context {
    text_size: usize,
    fn_name: String,
    template_params: Vec<(String, String)>,
    use_directives: HashSet<UseDirective>,
    diagnostic: Diagnostic,
}

impl Context {
    pub fn new(use_directives: HashSet<UseDirective>) -> Self {
        Self {
            use_directives,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct UseDirective {
    name: String,
    path: PathBuf,
    fn_name: String,
    params: Vec<(String, String)>,
    source: String,
}

impl Hash for UseDirective {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl PartialEq for UseDirective {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for UseDirective {}

pub fn expand(path: LitStr) -> TokenStream {
    let span = path.span();
    let path = path.value();

    let (ts, ctx) = compile(&path, span, Context::default());
    let mut use_directives = ctx.use_directives;

    let root_fn_name = Ident::new(&ctx.fn_name, Span::call_site());
    let root_fn_ts = quote! {self.#root_fn_name(__f__, |__f__: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {Ok(())})?;};
    // Ok(quote! {#root_fn_ts})

    let mut inner_use_directives = HashSet::new();

    // TODO: progress logic
    for use_directive in use_directives {
        let (ts, inner_ctx) = compile(
            use_directive.path.to_string_lossy().as_ref(),
            span,
            Context::new(use_directives.to_owned()),
        );

        inner_use_directives.extend(inner_ctx.use_directives);
    }

    todo!()
}

pub fn compile(path: &str, span: Span, mut ctx: Context) -> (TokenStream, Context) {
    let (full_path, input) = match read_template(&Path::new(&path)) {
        Ok((full_path, input)) => (full_path, input),
        Err(msg) => {
            return (
                quote_spanned! { span => compile_error!(#msg); },
                Context::default(),
            );
        }
    };

    let source = &input;

    ctx.diagnostic = Diagnostic::new(HashMap::from([(full_path.clone(), source.to_string())])); // TODO: remove clones and change the logic

    let body = {
        let mut input = Input {
            input: input.as_str(),
            state: &mut ctx,
        };

        let body = match template(&mut input) {
            Ok(res) => res,
            Err(e) => {
                let err = e.into_inner().unwrap();
                let msg = err
                    .context()
                    .filter_map(|c| match c {
                        StrContext::Label(l) => Some(format!("in {l}")),
                        StrContext::Expected(e) => match e {
                            StrContextValue::Description(desc) => Some(desc.to_string()),
                            other => Some(format!("expected {}", other)),
                        },
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join(": ");

                let offset = source.len().saturating_sub(input.input.len());
                let diag = show_error(&source, offset, &format!("{msg:?}"));

                let msg_lit = syn::LitStr::new(&diag, Span::call_site());

                quote::quote_spanned! { span =>
                    compile_error!(#msg_lit);
                }
            }
        };

        body
    };

    let full_path_str = full_path.to_string_lossy();

    let fn_name = generate_fn_name(&path);
    ctx.fn_name = fn_name.to_owned();

    let params_names: Vec<&str> = ctx
        .template_params
        .iter()
        .map(|(name, _)| name.as_str())
        .collect();

    let args = param_names_to_ts(&params_names);

    (
        quote! {
        let _ = include_str!(#full_path_str);
        fn #fn_name(&self,
                __f__: &mut dyn ::std::fmt::Write,
                child_content: impl Fn(&mut dyn ::std::fmt::Write) -> ::std::fmt::Result,
                #args) -> ::std::fmt::Result {#body Ok(())}
        },
        ctx,
    )
}

fn line_col(source: &str, byte_offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (i, ch) in source.char_indices() {
        if i >= byte_offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

fn show_error(source: &str, byte_offset: usize, msg: &str) -> String {
    let (line, col) = line_col(source, byte_offset);

    let line_start = source[..byte_offset]
        .rfind('\n')
        .map(|i| i + 1)
        .unwrap_or(0);
    let line_end = source[byte_offset..]
        .find('\n')
        .map(|i| byte_offset + i)
        .unwrap_or(source.len());

    let line_text = &source[line_start..line_end];

    let mut caret = String::new();
    caret.push_str(&" ".repeat(col.saturating_sub(1)));
    caret.push('^');

    format!("{msg}\n --> line {line}, col {col}\n{line_text}\n{caret}")
}

pub fn read_template(path: &Path) -> Result<(PathBuf, String), String> {
    let base_dir = match env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|_| env::current_dir())
    {
        Ok(base_dir) => base_dir,
        Err(e) => return Err(format!("Failed to get current directory: {}", e)),
    };

    let full_path = base_dir.join(path);

    let source = match fs::read_to_string(&full_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read '{}': {}", full_path.display(), e)),
    };

    Ok((full_path, source))
}

#[test]
fn test_rshtml_macro() {
    let result = compile(
        "views/rshtml_macro.rs.html",
        Span::call_site(),
        Context::default(),
    );
    println!("{0}", result.0);
}
