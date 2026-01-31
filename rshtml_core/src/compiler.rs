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
    path_stack: Vec<PathBuf>,
}

impl Compiler {
    pub fn new(struct_name: Ident, struct_generics: Generics, struct_fields: Vec<String>) -> Self {
        Compiler {
            struct_name,
            struct_generics,
            struct_fields: struct_fields,
            path_stack: Vec::new(),
        }
    }

    pub fn compile(&mut self, path: &Path) -> TokenStream {
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
        let path_buf = path.to_path_buf();
        if self.path_stack.contains(&path_buf) {
            return Err(format!("Circular dependency detected: {}", path.display()));
        }
        self.path_stack.push(path_buf);

        let mut ctx = Context::default();
        ctx.struct_fields = self.struct_fields.to_owned();

        let (mut fn_signs, mut fn_bodies, mut include_strs, ctx) =
            rshtml_file::compile(&path, ctx)?;
        let mut total_text_size = ctx.text_size;
        let fn_name = ctx.fn_name;

        for path in ctx
            .use_directives
            .iter()
            .map(|ud| ud.path.to_owned())
            .collect::<HashSet<PathBuf>>()
        {
            let (fn_sign, fn_body, include_str_ts, text_size, _) =
                self.compile_rshtml_files(&path)?;

            fn_bodies.extend(fn_body);
            fn_signs.extend(fn_sign);
            include_strs.extend(include_str_ts);
            total_text_size += text_size;
        }

        self.path_stack.pop();

        Ok((fn_signs, fn_bodies, include_strs, total_text_size, fn_name))
    }
}

#[test]
fn test_compiler() {
    let path = Path::new("views/rshtml_macro.rs.html");
    let ident = Ident::new("RsHtmlMacro", Span::call_site());
    let mut compiler = Compiler::new(ident, Generics::default(), vec!["user".to_owned()]);
    let result: TokenStream = compiler.compile(path);
    println!("{}", result.to_string());
}
