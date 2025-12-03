use crate::analyzer::Analyzer;
use crate::{
    node::{ComponentParameter, Node},
    position::Position,
};

pub struct ComponentAnalyzer;

impl ComponentAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        name: &String,
        _parameters: &Vec<ComponentParameter>,
        body: &[Node],
        position: &Position,
    ) -> Result<(), Vec<String>> {
        let has_child_content =
            if let Some((has_child_content, is_used)) = analyzer.components.get_mut(name) {
                *is_used = true;
                *has_child_content
            } else {
                let message = analyzer.message(
                    position,
                    "attempt to use a missing component",
                    &[],
                    &format!("component `{name}` is used but not found"),
                    name.len() + 1,
                );
                return Err(vec![message]);
            };

        if !analyzer.no_warn {
            if body.is_empty() && has_child_content {
                analyzer.warning(
                    position,
                    &format!("undefined body for component `<{name}>`"),
                    &[],
                    "`@child_content` is used, but the component body is undefined.",
                    name.len() + 1,
                );
            } else if !body.is_empty() && !has_child_content {
                analyzer.warning(
                    position,
                    &format!("defined body for component `<{name}>`"),
                    &[],
                    "`@child_content` is not used, but the component body is defined.",
                    name.len() + 1,
                );
            }
        }

        Ok(())
    }
}
