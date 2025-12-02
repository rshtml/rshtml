use crate::analyzer::Analyzer;
use crate::{
    node::{ComponentParameter, Node},
    position::Position,
};
use std::collections::HashMap;

pub struct ComponentAnalyzer;

impl ComponentAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        name: &String,
        parameters: &Vec<ComponentParameter>,
        body: &[Node],
        position: &Position,
    ) -> Result<(), Vec<String>> {
        let component = if let Some((component, is_used)) = analyzer.components.get_mut(name) {
            *is_used = true;
            component.to_owned()
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

        let params = &component.parameters;
        let code_block_vars = &component.code_block_vars;
        let has_child_content = &component.has_child_content;

        let mut params: HashMap<&String, bool> = params.iter().map(|x| (x, false)).collect();
        let mut params_extra = Vec::new();

        for parameter in parameters {
            match params.get_mut(&parameter.name) {
                Some(has) => *has = true,
                None => params_extra.push((&parameter.name, &parameter.position)),
            }
        }

        if !analyzer.no_warn {
            if !params_extra.is_empty() {
                let unused = params_extra
                    .iter()
                    .map(|s| format!("`{}`", s.0))
                    .collect::<Vec<_>>()
                    .join(", ");
                let p = if params_extra.len() > 1 {
                    "parameters"
                } else {
                    "parameter"
                };

                let mut lines: Vec<usize> = Vec::new();
                params_extra
                    .iter()
                    .for_each(|(_, pos)| lines.push((pos.0).0));

                analyzer.warning(
                    position,
                    &format!("unused {p} {unused} for component `<{name}>`"),
                    &lines,
                    "",
                    0,
                );
            }

            if body.is_empty() && *has_child_content {
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

        let params_missing: Vec<&String> = params
            .iter()
            .filter(|(name, has)| !*has && !code_block_vars.contains(name))
            .map(|(&name, _)| name)
            .collect();

        if !params_missing.is_empty() {
            let missing = params_missing
                .iter()
                .map(|s| format!("`{}`", s))
                .collect::<Vec<_>>()
                .join(", ");
            let p = if params_missing.len() > 1 {
                "parameters"
            } else {
                "parameter"
            };

            let message = analyzer.message(
                position,
                "",
                &[],
                &format!("{missing} {p} not found for this component"),
                name.len() + 1,
            );

            return Err(vec![message]);
        }

        Ok(())
    }
}
