use crate::{analyzer::Analyzer, diagnostic::Level, position::Position};
use syn::{Expr, parse_str, visit::Visit};

pub struct ExprAnalyzer<'a>(&'a Vec<String>, String, usize, bool);

impl<'a> ExprAnalyzer<'a> {
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

        let expression = match parse_str::<Expr>(expr) {
            Ok(expression) => expression,
            Err(e) => {
                analyzer.diagnostic(
                    position,
                    "attempt to use invalid expression",
                    &[],
                    &e.to_string(),
                    expr.len() + !*is_escaped as usize,
                    Level::Caution,
                );
                return;
            }
        };

        let mut visitor = ExprAnalyzer(
            &analyzer
                .component
                .fns
                .iter()
                .map(|f| f.name.to_owned())
                .collect(),
            String::new(),
            0,
            false,
        );
        visitor.visit_expr(&expression);

        let fn_name = visitor.1;
        let args_len = visitor.2;
        let is_fn = visitor.3;

        if is_fn
            && let Some(f) = analyzer.component.fns.iter().find(|f| f.name == fn_name)
            && f.params.len() != args_len
        {
            let message = format!(
                "expected {} parameter but found {args_len} parameter",
                f.params.len()
            );

            analyzer.diagnostic(
                position,
                "inconsistent number of function parameters",
                &[],
                &message,
                expr.len() + !*is_escaped as usize,
                Level::Caution,
            );
        }
    }
}

impl<'a> Visit<'_> for ExprAnalyzer<'a> {
    fn visit_expr(&mut self, node: &Expr) {
        if let Expr::Call(expr_call) = node
            && let Expr::Path(ref func_path) = *expr_call.func
            && func_path.path.segments.len() == 1
        {
            let ident = &func_path.path.segments[0].ident;

            if self.0.contains(&ident.to_string()) {
                let args_len = &expr_call.args.len();

                self.1 = ident.to_string();
                self.2 = args_len.to_owned();
                self.3 = true;
            }
        }
    }
}
