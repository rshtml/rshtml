use crate::rshtml_macro::{
    Input,
    extensions::ParserDiagnostic,
    template::{component_tag_identifier, string_line},
};
use proc_macro2::TokenStream;
use std::path::{Path, PathBuf};
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

            let path = Path::new(&path_str);

            let name = name_opt
                .map(|(_, _, name)| name.to_string())
                .or(extract_component_name(&path));

            (name, PathBuf::from(path))
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

    input.state.use_directives.push((name, path));

    Ok(TokenStream::new())
}

fn extract_component_name(path: &Path) -> Option<String> {
    let filename = path.file_name().and_then(|n| n.to_str())?;
    let component_name = filename.strip_suffix(".rs.html").unwrap_or(filename);
    Some(component_name.to_owned())
}
