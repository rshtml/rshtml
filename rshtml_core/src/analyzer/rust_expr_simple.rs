use crate::{analyzer::Analyzer, position::Position};
use anyhow::Result;

pub struct RustExprSimpleAnalyzer;

impl RustExprSimpleAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        expr: &str,
        _is_escaped: &bool,
        _position: &Position,
    ) -> Result<()> {
        if let Some(name) = &analyzer.is_component
            && Self::is_valid_attribute_name(expr)
        {
            analyzer
                .components
                .entry(name.to_owned())
                .and_modify(|component| component.parameters.push(expr.to_owned()));
        }

        Ok(())
    }

    fn is_valid_attribute_name(expr: &str) -> bool {
        let mut chars = expr.chars();

        match chars.next() {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
            }
            _ => false,
        }
    }
}
