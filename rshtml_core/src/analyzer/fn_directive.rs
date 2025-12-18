use syn::{Ident, Type, parse_str};

use crate::{analyzer::Analyzer, diagnostic::Level, node::Node, position::Position};

pub struct FnDirectiveAnalyzer;

impl FnDirectiveAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        name: &str,
        params: &Vec<(String, String, Position)>,
        body: &Vec<Node>,
        position: &Position,
    ) {
        if let Err(e) = parse_str::<Ident>(name) {
            analyzer.diagnostic(
                position,
                "attempt to use invalid identifier",
                &[],
                &format!("invalid function name `{name}`, {}", e.to_string()),
                name.len(),
                Level::Caution,
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

        for b in body {
            analyzer.analyze(b);
        }
    }
}
