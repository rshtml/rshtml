use crate::{analyzer::Analyzer, diagnostic::Level, position::Position};
use syn::{Ident, Type, parse_str};

pub struct TemplateParamsAnalyzer;

impl TemplateParamsAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        params: &Vec<(String, String, Position)>,
        _position: &Position,
    ) {
        if let Some(name) = &analyzer.is_component
            && let Some(component) = analyzer.components.get_mut(name)
        {
            component.parameters.extend(
                params
                    .iter()
                    .map(|p| p.0.to_owned())
                    .collect::<Vec<String>>(),
            );
        }

        for (param_name, param_type, param_position) in params {
            if parse_str::<Ident>(param_name).is_err() {
                analyzer.diagnostic(
                    param_position,
                    "attempt to use invalid identifier",
                    &[],
                    &format!("invalid parameter name `{param_name}`"),
                    param_name.len(),
                    Level::Caution,
                );
            }

            if parse_str::<Type>(param_type).is_err() {
                analyzer.diagnostic(
                    param_position,
                    "attempt to use invalid type",
                    &[],
                    &format!("invalid parameter type `{param_type}`"),
                    param_name.len(),
                    Level::Caution,
                );
            }
        }
    }
}
