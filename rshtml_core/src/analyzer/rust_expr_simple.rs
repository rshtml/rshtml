use crate::{analyzer::Analyzer, position::Position};

pub struct RustExprSimpleAnalyzer;

impl RustExprSimpleAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        expr: &str,
        is_escaped: &bool,
        position: &Position,
    ) -> Result<(), Vec<String>> {
        if let Some(name) = &analyzer.is_component
            && Self::is_valid_attribute_name(expr)
        {
            analyzer
                .components
                .entry(name.to_owned())
                .and_modify(|(component, _)| component.parameters.push(expr.to_owned()));
        }

        if let Some(field) = analyzer.get_struct_field(expr)
            && !analyzer.struct_fields.contains(&field)
        {
            analyzer.caution(
                position,
                "attempt to use undefined struct field",
                &[],
                " ",
                expr.len() + !*is_escaped as usize,
            );
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
