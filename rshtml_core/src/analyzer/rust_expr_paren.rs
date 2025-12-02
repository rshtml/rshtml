use crate::{analyzer::Analyzer, position::Position};

pub struct RustExprParenAnalyzer;

impl RustExprParenAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        expr: &str,
        is_escaped: &bool,
        position: &Position,
    ) -> Result<(), Vec<String>> {
        let expr_trimed = expr
            .strip_prefix('(')
            .and_then(|sub| sub.strip_suffix(')'))
            .unwrap_or(expr);

        if let Some(field) = analyzer.get_struct_field(expr_trimed)
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
}
