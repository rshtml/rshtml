use crate::analyzer::Analyzer;
use crate::{
    node::{ComponentParameter, Node},
    position::Position,
};
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashMap;

pub struct ComponentAnalyzer;

impl ComponentAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        name: &String,
        parameters: &Vec<ComponentParameter>,
        body: &Vec<Node>,
        position: &Position,
    ) -> Result<()> {
        analyzer.position = position.clone();

        let component = analyzer
            .components
            .get(name)
            .ok_or(anyhow!("Component {} not found", name))?;

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
                    &format!("unused {p} {unused} for component `<{name}>`"),
                    &lines,
                    "",
                    0,
                );
            }

            if body.is_empty() && *has_child_content {
                analyzer.warning(
                    &format!("undefined body for component `<{name}>`"),
                    &[],
                    &format!("`@child_content` is used, but the component body is undefined."),
                    name.len(),
                );
            } else if !body.is_empty() && !has_child_content {
                analyzer.warning(
                    &format!("defined body for component `<{name}>`"),
                    &[],
                    &format!("`@child_content` is not used, but the component body is defined."),
                    name.len(),
                );
            }
        }

        // let params_missing: Vec<&String> = params
        //     .iter()
        //     .filter(|(name, has)| !*has && !code_block_vars.contains(name))
        //     .map(|(&name, _)| name)
        //     .collect();

        // if !params_missing.is_empty() {
        //     return Err(anyhow!(Self::missing_params_error(
        //         compiler,
        //         params_missing,
        //         &position,
        //         name.len()
        //     )));
        // }

        Ok(())
    }

    // fn missing_params_error(
    //     compiler: &mut Compiler,
    //     params_missing: Vec<&String>,
    //     position: &Position,
    //     name_len: usize,
    // ) -> String {
    //     let file_info = compiler.files_to_info(position);
    //     let source_first_line: String = compiler.source_first_line(position).unwrap_or_default();
    //     let hyphen = "-".repeat(name_len + 1);
    //     let missing = params_missing
    //         .iter()
    //         .map(|s| format!("`{}`", s))
    //         .collect::<Vec<_>>()
    //         .join(", ");
    //     let p = if params_missing.len() > 1 {
    //         "parameters"
    //     } else {
    //         "parameter"
    //     };

    //     let line_num = (position.0).0;
    //     let left_pad = line_num.to_string().len();
    //     let left_pad = " ".repeat(left_pad);

    //     return format!(
    //         "{left_pad} --> {file_info}\n{left_pad} |\n{line_num} | {source_first_line}\n{left_pad} | {hyphen} {missing} {p} not found for this component\n{left_pad} |",
    //     );
    // }
}
