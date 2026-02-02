use crate::{context::Context, rshtml_file};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
use syn::{Generics, Ident};

pub struct Compiler {
    struct_name: Ident,
    struct_generics: Generics,
    struct_fields: Vec<String>,
    base_dir: PathBuf,
    path_stack: Vec<PathBuf>,
}

impl Compiler {
    pub fn new(struct_name: Ident, struct_generics: Generics, struct_fields: Vec<String>) -> Self {
        Compiler {
            struct_name,
            struct_generics,
            struct_fields: struct_fields,
            base_dir: PathBuf::new(),
            path_stack: Vec::new(),
        }
    }

    pub fn compile(&mut self, path: &Path) -> TokenStream {
        self.base_dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let Some(path) = path.file_name().map(Path::new) else {
            return quote_spanned! { self.struct_name.span() => compile_error!("Invalid path: {}", path.display()); };
        };

        let (fn_signs, fn_bodies, include_strs, total_text_size, fn_name) = match self
            .compile_rshtml_files(&path)
        {
            Ok((fn_signs, fn_bodies, include_strs, total_text_size, fn_name)) => {
                (fn_signs, fn_bodies, include_strs, total_text_size, fn_name)
            }
            Err(err) => {
                let error_message = format!(
                    "Template processing failed for struct `{}` with template `{}`:\n{err}",
                    self.struct_name,
                    path.display()
                );

                return quote_spanned! { self.struct_name.span() => compile_error!(#error_message); };
            }
        };

        let root_fn_name = Ident::new(&fn_name, Span::call_site());
        let root_fn_call = quote! {self.#root_fn_name(__out__, |__out__: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {Ok(())})?;};

        let (impl_generics, type_generics, where_clause) = self.struct_generics.split_for_impl();
        let struct_name = self.struct_name.to_owned();

        quote! {
             const _ : () = {
                #include_strs

                impl #impl_generics ::rshtml::traits::View for #struct_name #type_generics #where_clause {
                    fn render(&self, __out__: &mut dyn ::std::fmt::Write) -> ::std::fmt::Result {
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
                        #total_text_size
                    }
                }
            };
        }
    }

    fn compile_rshtml_files(
        &mut self,
        path: &Path,
    ) -> Result<(TokenStream, TokenStream, TokenStream, usize, String), String> {
        if let Some(start_index) = self.path_stack.iter().position(|p| p == path) {
            let mut chain: Vec<String> = self.path_stack[start_index..]
                .iter()
                .map(|p| p.display().to_string())
                .collect();
            chain.push(path.display().to_string());

            let chain_str = chain.join(" -> ");

            return Err(format!("Circular dependency detected: {}", chain_str));
        }

        // if self.path_stack.iter().any(|p| p == path) {
        //     return Err(format!("Circular dependency detected: {}", path.display()));
        // }

        self.path_stack.push(path.to_path_buf());

        let mut ctx = Context::default();
        ctx.struct_fields = self.struct_fields.to_owned();
        ctx.base_dir = self.base_dir.to_owned();

        let path_with_base = self.base_dir.join(path);
        let (mut fn_signs, mut fn_bodies, mut include_strs, ctx) =
            rshtml_file::compile(&path_with_base, ctx)?;
        let mut total_text_size = ctx.text_size;
        let fn_name = ctx.fn_name;

        for p in ctx
            .use_directives
            .iter()
            .map(|ud| ud.path.to_owned())
            .collect::<HashSet<PathBuf>>()
        {
            let (fn_sign, fn_body, include_str_ts, text_size, _) = self
                .compile_rshtml_files(&p)
                .map_err(|e| format!("{}:\n{}", path_with_base.display(), e))?;

            fn_bodies.extend(fn_body);
            fn_signs.extend(fn_sign);
            include_strs.extend(include_str_ts);
            total_text_size += text_size;
        }

        self.path_stack.pop();

        Ok((fn_signs, fn_bodies, include_strs, total_text_size, fn_name))
    }
}
