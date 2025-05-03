use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::iterators::Pair;

pub struct IncludeDirectiveParser;

impl IParser for IncludeDirectiveParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, String> {
        let path_pair = pair.into_inner().find(|p| p.as_rule() == Rule::string_line).unwrap();

        let path = path_pair.as_str().trim_matches('"').trim_matches('\'').to_string();

        let view_path = parser.config.views_base_path.join(&path);

        let included_content = match std::fs::read_to_string(&view_path) {
            Ok(content) => content,
            Err(e) => {
                return Err(format!("Error reading included file '{}': {}", path, e));
            }
        };

        let canonical_path = view_path.canonicalize().unwrap_or_default().to_string_lossy().to_string();

        if parser.included_templates.contains(&canonical_path) {
            return Err(format!("Error: Circular include detected for file '{}'", path));
        }

        let mut included_templates = parser.included_templates.clone();
        included_templates.insert(canonical_path);

        let inner_template = parser.parse_template(included_content.clone().as_str())?;

        let nodes = match inner_template {
            Node::Template(nodes) => nodes,
            _ => {
                return Err(format!("Error: Expected a template in the included file '{}', found {:?}", path, inner_template));
            }
        };

        Ok(Node::Template(nodes))
    }
}
