use crate::analyzer::Analyzer;
use crate::diagnostic::Level;
use crate::node::ComponentParameterValue;
use crate::{
    node::{ComponentParameter, Node},
    position::Position,
};
use std::path::PathBuf;

pub struct ComponentAnalyzer;

impl ComponentAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        name: &String,
        parameters: &[ComponentParameter],
        body: &[Node],
        position: &Position,
    ) {
        let mut found = false;
        let mut component_path = PathBuf::new();

        analyzer
            .component
            .use_directives
            .iter_mut()
            .filter(|ud| &ud.name == name)
            .for_each(|ud| {
                found = true;
                component_path = ud.path.to_owned();
                ud.is_used = true;
            });

        let (missing_params, missing_len, extra_params, has_child_content) = if let Some(component) =
            analyzer.components.get(&component_path)
            && found
        {
            let params = parameters
                .iter()
                .map(|p| p.name.as_str())
                .collect::<Vec<&str>>();

            let mut missing_len = 0;
            let missing_params = component
                .parameters
                .iter()
                .filter(|p| !params.contains(&p.as_str()))
                .fold(String::new(), |mut acc, p| {
                    missing_len += 1;

                    if !acc.is_empty() {
                        acc.push_str(", ");
                    }
                    acc.push('`');
                    acc.push_str(p);
                    acc.push('`');
                    acc
                });

            let extra_params = parameters
                .iter()
                .filter(|p| !component.parameters.contains(&p.name))
                .collect::<Vec<&ComponentParameter>>();

            (
                missing_params,
                missing_len,
                extra_params,
                component.has_child_content,
            )
        } else {
            analyzer.diagnostic(
                position,
                "attempt to use a missing component",
                &[],
                &format!("component `{name}` is used but not found"),
                name.len() + 1,
                Level::Caution,
            );

            return;
        };

        for parameter in parameters {
            match &parameter.value {
                ComponentParameterValue::Bool(_value) => {}
                ComponentParameterValue::Number(_value) => {}
                ComponentParameterValue::String(_value) => {}
                ComponentParameterValue::RustExprParen(value) => analyzer.analyze(&Node::Expr(
                    value.to_owned(),
                    false,
                    parameter.position.to_owned(),
                )),
                ComponentParameterValue::RustExprSimple(value) => analyzer.analyze(&Node::Expr(
                    value.to_owned(),
                    false,
                    parameter.position.to_owned(),
                )),
                ComponentParameterValue::Block(value) => {
                    for v in value {
                        analyzer.analyze(v);
                    }
                }
            }
        }

        for b in body {
            analyzer.analyze(b);
        }

        if !missing_params.is_empty() {
            let s = if missing_len > 1 { "s" } else { "" };
            analyzer.diagnostic(
                position,
                &format!("missing component parameter{s} {missing_params}"),
                &[],
                " ",
                0,
                Level::Caution,
            );
        }

        if !analyzer.no_warn {
            let (extra_names, extra_lines) = extra_params.iter().fold(
                (String::new(), Vec::new()),
                |(mut acc_name, mut acc_pos), p| {
                    if !acc_name.is_empty() {
                        acc_name.push_str(", ");
                    }
                    acc_name.push('`');
                    acc_name.push_str(&p.name);
                    acc_name.push('`');

                    acc_pos.push(p.position.0.0);

                    (acc_name, acc_pos)
                },
            );

            if !extra_names.is_empty() {
                let s = if extra_lines.len() > 1 { "s" } else { "" };
                analyzer.diagnostic(
                    position,
                    &format!("unused component parameter{s} {extra_names}"),
                    &extra_lines,
                    " ",
                    0,
                    Level::Warning,
                );
            }

            if body.is_empty() && has_child_content {
                analyzer.diagnostic(
                    position,
                    &format!("undefined body for component `<{name}>`"),
                    &[],
                    "`@child_content` is used, but the component body is undefined.",
                    name.len() + 1,
                    Level::Warning,
                );
            } else if !body.is_empty() && !has_child_content {
                analyzer.diagnostic(
                    position,
                    &format!("defined body for component `<{name}>`"),
                    &[],
                    "`@child_content` is not used, but the component body is defined.",
                    name.len() + 1,
                    Level::Warning,
                );
            }
        }
    }
}
