use crate::Node;
use crate::config::Config;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::iterators::Pair;
use std::collections::HashSet;
use std::path::Path;

pub struct UseDirectiveParser;

impl IParser for UseDirectiveParser {
    fn parse(_: &RsHtmlParser, pair: Pair<Rule>, _: &Config, _: &HashSet<String>) -> Result<Node, String> {
        let mut inner_pairs = pair.into_inner();
        let import_path_str = inner_pairs.find(|p| p.as_rule() == Rule::string_line).unwrap().as_str().trim_matches('"').to_string();
        let import_path = Path::new(&import_path_str);
        import_path
            .file_name()
            .and_then(|name| name.to_str())
            .filter(|name| name.ends_with(".rs.html"))
            .ok_or_else(|| format!("Failed to derive component name from import path extension: '{:#?}'. Expected format like 'name.rs.html'.", import_path))?;

        let component_name = match inner_pairs.find(|p| p.as_rule() == Rule::rust_identifier) {
            Some(component_name_pair) => component_name_pair.as_str().to_string(),
            None => import_path
                .file_stem()
                .and_then(|stem1| Path::new(stem1).file_stem())
                .and_then(|stem2| stem2.to_str())
                .map(|s| s.to_string())
                .ok_or_else(|| format!("Failed to derive component name from import path: '{:#?}'", import_path))?,
        };

        Ok(Node::UseDirective(component_name, import_path.to_path_buf()))
    }
}
