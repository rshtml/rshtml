use crate::{analyzer::Analyzer, position::Position};

pub struct RustExprSimpleAnalyzer;

impl RustExprSimpleAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer, expr: &str, is_escaped: &bool, position: &Position) {
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
    }
}
