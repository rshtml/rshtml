use syn::{Expr, parse_str};

use crate::{analyzer::Analyzer, diagnostic::Level, position::Position};

pub struct ExprAnalyzer;

impl ExprAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer, expr: &str, is_escaped: &bool, position: &Position) {
        if let Some(field) = analyzer.get_struct_field(expr)
            && !analyzer.struct_fields.contains(&field)
        {
            analyzer.diagnostic(
                position,
                "attempt to use undefined struct field",
                &[],
                " ",
                expr.len() + !*is_escaped as usize,
                Level::Caution,
            );
        }

        if let Err(e) = parse_str::<Expr>(expr) {
            analyzer.diagnostic(
                position,
                "attempt to use invalid expression",
                &[],
                &e.to_string(),
                expr.len() + !*is_escaped as usize,
                Level::Caution,
            );
        }
    }
}
