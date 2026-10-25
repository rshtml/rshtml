mod break_directive;
mod child_content_directive;
mod codegen;
mod component;
mod continue_directive;
mod inner_text;
mod rust_block;
mod rust_stmt;
mod simple_expr;
mod simple_expr_paren;
mod template;
mod template_params;
mod text;
mod use_directive;
pub mod utils;

use crate::{
    context::{Context, Info, Op, UseDirective},
    debug,
    diagnostic::Diagnostic,
    extract_file,
    position::Position,
    rshtml_file::utils::params_to_fn_call_ts,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::{
    collections::{BTreeMap, HashSet},
    env, fs, mem,
    path::{Path, PathBuf},
    str::FromStr,
};
use syn::Ident;
use template::template;
use utils::{generate_fn_name, params_to_ts};
use winnow::{
    LocatingSlice, Stateful,
    error::{StrContext, StrContextValue},
};

pub type Input<'a, 'ctx> = Stateful<LocatingSlice<&'a str>, &'ctx mut Context<'a>>;

pub fn parse(path: &Path, base_dir: &Path) -> Result<(Vec<Op>, Info, String), String> {
    let (full_path, mut source) = match read_template(path) {
        Ok((full_path, source)) => (full_path, source),
        Err(msg) => {
            return Err(msg);
        }
    };

    if source.starts_with('\u{FEFF}') {
        source.drain(..3);
    }

    let mut ctx = Context::new(base_dir);
    ctx.info.fn_name = generate_fn_name(path);
    ctx.info.full_path = full_path;

    let mut input = Input {
        input: LocatingSlice::new(&source),
        state: &mut ctx,
    };

    if let Err(e) = template(&mut input) {
        let err = e.into_inner().unwrap();

        let mut labels = Vec::new();
        let mut expecteds = Vec::new();

        for context in err.context() {
            match context {
                StrContext::Label(l) => labels.push(l.to_string()),
                StrContext::Expected(e) => match e {
                    StrContextValue::Description(desc) => expecteds.push(desc.to_string()),
                    other => expecteds.push(format!("expected {}", other)),
                },
                _ => {}
            }
        }

        let label = if !labels.is_empty() {
            labels.reverse();
            format!("in {}:", labels.join(" > "))
        } else {
            String::new()
        };

        expecteds.reverse();
        let expected = expecteds.join(" or ");

        let offset = source.len().saturating_sub(input.input.len());
        let position: Position = (source.as_str(), offset).into();
        let diag = Diagnostic(&source).message(path, &position, &label, &expected, 1);

        return Err(diag);
    }

    Ok((ctx.ops, ctx.info, source))
}

pub fn extract(path: &Path, source: &str, ops: &[Op]) -> Result<PathBuf, String> {
    let mut debug_extract = debug::Debug::new(
        source
            .as_bytes()
            .iter()
            .map(|&b| if b == b'\n' { b'\n' } else { b' ' })
            .collect::<Vec<u8>>(),
    );

    debug_extract.process(source, ops);

    let extract_source = format!(
        "{1} {0}",
        " Ok(()) } use super::*;use ::rshtml::{Render, View};use std::fmt::Display;",
        String::from_utf8_lossy(&debug_extract.cleaned_source)
    );

    extract_file::create(path, &extract_source)
        .map_err(|e| format!("Failed to create extract file:({}) : {}", path.display(), e))
}

pub fn codegen(
    path: &Path,
    fn_name: &str,
    use_directives: &HashSet<UseDirective>,
    ops: Vec<Op>,
    source: &str,
    is_root: bool,
) -> Result<
    (
        TokenStream,
        TokenStream,
        TokenStream,
        Option<TokenStream>,
        usize,
    ),
    String,
> {
    let full_path_str = path.to_string_lossy();

    let mut params: Vec<(&str, &str)> = Vec::new();
    let mut text_size = 0;

    fn flush_view_fn_attr<'a>(
        current_attr_name: &mut Option<&'a str>,
        attrs: &mut BTreeMap<&'a str, TokenStream>,
        current: &mut TokenStream,
    ) {
        if let Some(prev_name) = current_attr_name.take() {
            let value = mem::take(current);
            let value = quote! {
                ::rshtml::ViewFn::new((|__out__: &mut dyn ::rshtml::Write| -> ::std::fmt::Result { #value Ok(()) }, 0))
            };
            attrs.insert(prev_name, value);
        }
    }

    enum Scope<'a> {
        Code(TokenStream),
        Component {
            name: &'a str,
            attrs: BTreeMap<&'a str, TokenStream>,
            current_attr_name: Option<&'a str>,
            in_body: bool,
        },
    }

    let mut current = TokenStream::new();
    let mut stack: Vec<TokenStream> = Vec::new();
    let mut comp_stack: Vec<ComponentFrame> = Vec::new();

    for op in ops {
        match op {
            Op::TemplateParam(range) => {
                codegen::template_param(source, range, &mut params);
            }
            Op::OpenBrace(_) => stack.push(Scope::Code(mem::take(&mut current))),
            Op::CloseBrace(_) => {
                let Some(Scope::Code(mut parent)) = stack.pop() else {
                    return Err("Unmatched closing brace '}'".to_string());
                };
                parent.extend(quote! { { #current } });
                current = parent;
            }
            Op::RustCode(range) => {
                let ts = codegen::rust_code(source, range)?;
                current.extend(ts);
            }
            Op::Text(range) => {
                let (ts, size) = codegen::text(source, range);
                text_size += size;
                current.extend(ts);
            }
            Op::ChildContent(_) => {
                let ts = codegen::child_content();
                current.extend(ts);
            }
            Op::ContinueOrBreak(range) => {
                let ts = codegen::continue_or_break(source, range);
                current.extend(ts);
            }
            Op::ExprSimple(range, is_escaped) => {
                let ts = codegen::expr_simple_or_paren(source, range, is_escaped, path)?;
                current.extend(ts);
            }
            Op::ExprParen(range, is_escaped) => {
                let ts = codegen::expr_simple_or_paren(source, range, is_escaped, path)?;
                current.extend(ts);
            }
            Op::ComponentStartName(range) => {
                stack.push(Scope::Code(mem::take(&mut current)));
                stack.push(Scope::Component {
                    name: &source[range],
                    attrs: BTreeMap::new(),
                    current_attr_name: None,
                    in_body: false,
                });
            }
            Op::ComponentAttribute(name_range, val_range_opt) => {
                let Some(Scope::Component {
                    attrs,
                    current_attr_name,
                    ..
                }) = stack.last_mut()
                else {
                    return Err("Invalid Component".to_string());
                };

                flush_view_fn_attr(current_attr_name, attrs, &mut current);

                let attr_name = &source[name_range];
                match val_range_opt {
                    Some(val_range) => {
                        let ts =
                            TokenStream::from_str(&source[val_range]).map_err(|e| e.to_string())?;
                        attrs.insert(attr_name, ts);
                    }
                    None => {
                        *current_attr_name = Some(attr_name);
                    }
                }
            }
            Op::ComponentBodyStart => {
                let Some(Scope::Component {
                    attrs,
                    current_attr_name,
                    in_body,
                    ..
                }) = stack.last_mut()
                else {
                    return Err("Invalid Component".to_string());
                };
                flush_view_fn_attr(current_attr_name, attrs, &mut current);
                *in_body = true;
            }
            Op::ComponentEnd => {
                let Some(Scope::Component {
                    name,
                    mut attrs,
                    mut current_attr_name,
                    in_body,
                }) = stack.pop()
                else {
                    return Err("Unmatched closing component tag".to_string());
                };
                let Some(Scope::Code(mut parent)) = stack.pop() else {
                    return Err("Unmatched closing component tag".to_string());
                };

                if !in_body {
                    flush_view_fn_attr(&mut current_attr_name, &mut attrs, &mut current);
                }

                let child_content = quote! {let child_content = |__out__: &mut dyn ::rshtml::Write| -> ::std::fmt::Result {#current  Ok(())};};

                let use_directive = use_directives
                    .iter()
                    .find(|use_directive| use_directive.name == name)
                    .ok_or(format!(
                        "attempt to use a missing component: component `{}` is used but not found",
                        name
                    ))?;

                let values = attrs.values();
                let args = quote! { #(#values),* };
                let comp_fn_name = Ident::new(&use_directive.fn_name, Span::call_site());
                let fn_call = quote! {Self::#comp_fn_name(__out__, child_content, #args)?;};

                parent.extend(quote! {
                    {
                        #child_content
                        #fn_call
                    }
                });

                current = parent;
            }
            _ => {}
        }
    }

    if !stack.is_empty() {
        return Err("Unclosed opening brace '{'".to_string());
    }

    let body = current;

    let args = params_to_ts(&mut params);
    let fn_name = Ident::new(fn_name, Span::call_site());

    let mut root_fn = None;
    if is_root {
        let root_fn_name = &fn_name;
        let root_fn_args = params_to_fn_call_ts(&mut params);
        let root_fn_call = quote! {Self::#root_fn_name(__out__, |__out__: &mut dyn ::rshtml::Write| -> ::std::fmt::Result {Ok(())}, #root_fn_args)?;};
        root_fn = Some(root_fn_call);
    }

    Ok((
        quote! {
        fn #fn_name(__out__: &mut dyn ::rshtml::Write,
                child_content: impl Fn(&mut dyn ::rshtml::Write) -> ::std::fmt::Result,
                #args) -> ::std::fmt::Result;
        },
        quote! {
        fn #fn_name(__out__: &mut dyn ::rshtml::Write,
                child_content: impl Fn(&mut dyn ::rshtml::Write) -> ::std::fmt::Result,
                #args) -> ::std::fmt::Result {#body Ok(())}
        },
        quote! { let _ = include_str!(#full_path_str); },
        root_fn,
        text_size,
    ))
}

fn read_template(path: &Path) -> Result<(PathBuf, String), String> {
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
