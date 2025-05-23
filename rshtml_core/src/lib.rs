mod compiler;
mod config;
mod error;
pub mod functions;
mod node;
mod parser;
#[cfg(test)]
mod tests;

use crate::config::Config;
use crate::parser::RsHtmlParser;
use anyhow::Result;
use node::Node;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};

pub fn process_template(template_name: String, struct_name: &Ident) -> TokenStream {
    let config = Config::load_from_toml_or_default();
    let layout = config.layout.clone();
    let locales_base_path = config.locales_base_path.clone();
    let locales_base_path = locales_base_path.to_string_lossy().into_owned();
    let locale_lang = config.locale_lang.clone();

    let (compiled_ast_tokens, sections) = match parse_and_compile(&template_name, config) {
        Ok(tokens) => tokens,
        Err(err) => {
            let error_message = format!(
                "Template processing failed for struct `{}` with template `{}`:\n{}",
                struct_name,
                template_name,
                err.to_string()
            );

            return quote_spanned! { struct_name.span() => compile_error!(#error_message); }.into();
        }
    };

    //dbg!("DEBUG: Generated write_calls TokenStream:\n{}", compiled_ast_tokens.to_string());

    let generated_code = quote! {
        const _ : () = {
            static __rs__: ::std::sync::LazyLock<::std::sync::RwLock<rshtml::Functions>> = ::std::sync::LazyLock::new(|| ::std::sync::RwLock::new(rshtml::Functions::new(#layout.to_string(), #sections, #locales_base_path, #locale_lang)));

            impl ::std::fmt::Display for #struct_name {
                 fn fmt(&self, __f__: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    let mut rs = __rs__.write().map_err(|err| {
                        eprintln!("ERROR: RsHtml functions was poisoned during Display::fmt!: {err:?}");
                        std::fmt::Error
                    })?;

                    #compiled_ast_tokens

                    Ok(())
                 }
            }
        };
    };

    TokenStream::from(generated_code)
}

fn parse_and_compile(template_path: &str, config: Config) -> Result<(TokenStream, TokenStream)> {
    let mut rshtml_parser = RsHtmlParser::new();
    let node = rshtml_parser.run(template_path, config)?;

    let mut compiler = compiler::Compiler::new();
    let ts = compiler.compile(&node)?;

    if let Some(layout) = compiler.layout.clone() {
        compiler.section_body = Some(ts.clone());
        let layout_ts = compiler.compile(&layout)?;

        return Ok((layout_ts, compiler.section_names()));
    }

    Ok((ts, compiler.section_names()))
}
