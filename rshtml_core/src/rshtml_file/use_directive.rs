use super::{
    Input, component::component_tag_identifier, template::string_line, utils::generate_fn_name,
};
use crate::{
    context::UseDirective, extensions::ParserDiagnostic, rshtml_file::utils::extract_component_name,
};
use proc_macro2::TokenStream;
use std::path::PathBuf;
use winnow::{
    ModalResult, Parser,
    ascii::{multispace0, multispace1},
    combinator::{alt, cut_err, opt},
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    stream::Stream,
};

pub fn use_directive<'a>(input: &mut Input<'a>) -> ModalResult<TokenStream> {
    let checkpoint = input.checkpoint();

    let (name, path) = (
        "use",
        alt((
            (multispace1, cut_err(string_line).expected("path")),
            (multispace0, string_line),
        ))
        .map(|(_, sl)| sl),
        multispace0,
        opt((
            "as",
            multispace0,
            cut_err(component_tag_identifier).expected("component tag identfier"),
        )),
        opt(';'),
    )
        .map(|(_, path_str, _, name_opt, _)| {
            let mut path_str = if path_str.starts_with('\'') {
                path_str.trim_matches('\'')
            } else {
                path_str.trim_matches('"')
            }
            .to_string();

            if !path_str.ends_with(".rs.html") {
                path_str.push_str(".rs.html");
            }

            let path = PathBuf::from(path_str);

            let name = name_opt
                .map(|(_, _, name)| name.to_string())
                .or(extract_component_name(&path));

            (name, path)
        })
        .parse_next(input)?;

    let name = match name {
        Some(name) => name,
        None => {
            input.reset(&checkpoint);

            return Err(ErrMode::Cut(ContextError::new().add_context(
                input,
                &checkpoint,
                StrContext::Expected(StrContextValue::Description("invalid component path")),
            )));
        }
    };

    let fn_name = generate_fn_name(&path);

    let path = path.to_path_buf();

    input.state.use_directives.insert(UseDirective {
        name,
        path,
        fn_name,
    });

    Ok(TokenStream::new())
}
