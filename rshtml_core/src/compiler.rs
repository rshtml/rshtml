use crate::{context::UseDirective, diagnostic::Diagnostic, extract_file, rshtml_file};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use std::{
    collections::{HashMap, HashSet},
    mem,
    path::{Path, PathBuf},
    str::FromStr,
};
use syn::{Generics, Ident};

#[derive(Default)]
struct CompileOutput {
    fn_signs: TokenStream,
    fn_bodies: TokenStream,
    include_strs: TokenStream,
    text_sizes: usize,
    fn_name: String,
}

pub struct Compiler {
    struct_name: Ident,
    struct_generics: Generics,
    struct_fields: Vec<String>,
    base_dir: PathBuf,
    path_stack: Vec<PathBuf>,
    visited_paths: HashMap<PathBuf, (HashSet<UseDirective>, String)>,
    extract: bool,

    extract_includes: TokenStream,
}

impl Compiler {
    pub fn new(
        struct_name: Ident,
        struct_generics: Generics,
        struct_fields: Vec<String>,
        extract: bool,
    ) -> Self {
        Compiler {
            struct_name,
            struct_generics,
            struct_fields,
            base_dir: PathBuf::new(),
            path_stack: Vec::new(),
            visited_paths: HashMap::new(),
            extract,

            extract_includes: TokenStream::new(),
        }
    }

    pub fn compile(&mut self, path: &Path) -> TokenStream {
        self.base_dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();

        let compile_output = match self.compile_rshtml_files(path) {
            Ok(compile_output) => compile_output,
            Err(err) => {
                let error_message = format!(
                    "Template processing failed for struct `{}` with template `{}`:\n{err}",
                    self.struct_name,
                    path.display()
                );

                let (impl_generics, type_generics, where_clause) =
                    self.struct_generics.split_for_impl();
                let struct_name = self.struct_name.to_owned();

                return quote_spanned! { self.struct_name.span() =>
                    compile_error!(#error_message);

                    impl #impl_generics ::rshtml::View for #struct_name #type_generics #where_clause {
                        fn render(&self, __out__: &mut dyn ::rshtml::Write) -> ::std::fmt::Result {
                            Ok(())
                        }

                        fn text_size(&self) -> usize {
                            0
                        }
                    }
                };
            }
        };

        let fn_signs = compile_output.fn_signs;
        let fn_bodies = compile_output.fn_bodies;
        let include_strs = compile_output.include_strs;
        let text_sizes = compile_output.text_sizes;

        let root_fn_name = Ident::new(&compile_output.fn_name, Span::call_site());
        let root_fn_call = quote! {self.#root_fn_name(__out__, |__out__: &mut dyn ::rshtml::Write| -> ::std::fmt::Result {Ok(())})?;};

        let (impl_generics, type_generics, where_clause) = self.struct_generics.split_for_impl();
        let struct_name = self.struct_name.to_owned();

        let mut extract_files_ts = TokenStream::new();
        if cfg!(debug_assertions) {
            let extract_includes = self.extract_includes.to_owned();
            extract_files_ts = quote! {
                impl #impl_generics #struct_name #type_generics #where_clause {
                    #extract_includes
                }
            };
        }

        quote! {
            #[allow(unused, path_statements, clippy::all)]
            const _ : () = {
                use ::rshtml::{Render, View};
                use ::std::fmt::Display;

                #include_strs

                #extract_files_ts

                impl #impl_generics ::rshtml::View for #struct_name #type_generics #where_clause {
                    fn render(&self, __out__: &mut dyn ::rshtml::Write) -> ::std::fmt::Result {
                        trait __rshtml__fns {
                            #fn_signs
                        }

                        impl #impl_generics __rshtml__fns for #struct_name #type_generics #where_clause {
                            #fn_bodies
                        }

                        #root_fn_call

                        Ok(())
                    }

                    fn text_size(&self) -> usize {
                        #text_sizes
                    }
                }
            };
        }
    }

    fn compile_rshtml_files(&mut self, path: &Path) -> Result<CompileOutput, String> {
        self.path_stack.push(path.to_path_buf());

        let mut compile_output = if !self.visited_paths.contains_key(path) {
            let (fn_signs, fn_bodies, include_strs, info, source) =
                rshtml_file::compile(path, &self.base_dir, &self.struct_fields, self.extract)?;

            self.visited_paths
                .entry(path.to_path_buf())
                .or_insert((info.use_directives, source));

            if cfg!(debug_assertions) && self.extract {
                let extract_file_include = extract_file::create(
                    &path.to_owned(),
                    &format!("{{ {} }}", String::from_utf8_lossy(&info.debug.source)),
                )
                .map_err(|e| e.to_string())?;

                let fn_name =
                    TokenStream::from_str(&format!("_rshtml_{0}", info.fn_name.to_owned()))
                        .unwrap();

                let fn_params = info
                    .template_params
                    .iter()
                    .map(|(n, t)| format!("{n}: {t}"))
                    .collect::<Vec<String>>()
                    .join(", ");
                let fn_params_ts = TokenStream::from_str(&fn_params).unwrap();

                let ts = quote! {
                    fn #fn_name (&self, child_content: impl ::rshtml::View, #fn_params_ts) -> ::std::fmt::Result {
                        #extract_file_include

                        Ok(())
                    }
                };

                self.extract_includes.extend(ts);
            }

            CompileOutput {
                fn_signs,
                fn_bodies,
                include_strs,
                text_sizes: info.text_size,
                fn_name: info.fn_name,
            }
        } else {
            CompileOutput::default()
        };

        let visited = self
            .visited_paths
            .get_mut(path)
            .map(mem::take)
            .unwrap_or_default();

        for use_directive in &visited.0 {
            if self.path_stack.iter().any(|p| p == &use_directive.path) {
                return Err(Diagnostic(&visited.1).message(
                    path,
                    &use_directive.position,
                    "in use directive",
                    "circular dependency detected",
                    1,
                ));
            }

            let output = self.compile_rshtml_files(&use_directive.path)?;

            compile_output.fn_bodies.extend(output.fn_bodies);
            compile_output.fn_signs.extend(output.fn_signs);
            compile_output.include_strs.extend(output.include_strs);
            compile_output.text_sizes += output.text_sizes;
        }

        if let Some(entry) = self.visited_paths.get_mut(path) {
            *entry = visited;
        }

        self.path_stack.pop();

        Ok(compile_output)
    }
}
