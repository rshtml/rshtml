mod child_content;
mod component;
mod expr;
mod match_expr;
mod rust_block;
mod rust_expr;
mod template;
mod template_params;
mod use_directive;

use crate::{
    analyzer::{
        child_content::ChildContentAnalyzer, component::ComponentAnalyzer, expr::ExprAnalyzer,
        match_expr::MatchExprAnalyzer, rust_block::RustBlockAnalyzer, rust_expr::RustExprAnalyzer,
        template::TemplateAnalyzer, template_params::TemplateParamsAnalyzer,
        use_directive::UseDirectiveAnalyzer,
    },
    diagnostic::{Diagnostic, Level},
    node::Node,
    position::Position,
};
use std::{collections::HashMap, path::PathBuf};
use syn::{Member, parse_str};

pub struct Analyzer {
    files: Vec<(String, Position)>,
    use_directives: Vec<(String, PathBuf, Position)>,
    components: HashMap<String, (bool, bool)>, // has_child_content, is_used
    layout: Option<Node>,
    no_warn: bool,
    is_component: Option<String>,
    struct_fields: Vec<String>,
    pub diagnostic: Diagnostic,
}

impl Analyzer {
    fn new(diagnostic: Diagnostic, struct_fields: Vec<String>, no_warn: bool) -> Self {
        Self {
            files: Vec::new(),
            use_directives: Vec::new(),
            components: HashMap::new(),
            layout: None,
            diagnostic,
            no_warn,
            is_component: None,
            struct_fields,
        }
    }

    fn analyze(&mut self, node: &Node) {
        match node {
            Node::Template(file, name, fn_names, nodes, position) => {
                TemplateAnalyzer::analyze(self, file, name, fn_names, nodes, position)
            }
            Node::Text(_) => (),
            Node::TemplateParams(params, position) => {
                TemplateParamsAnalyzer::analyze(self, params, position)
            }
            Node::RustBlock(content, position) => {
                RustBlockAnalyzer::analyze(self, content, position)
            }
            Node::Expr(expr, is_escaped, position) => {
                ExprAnalyzer::analyze(self, expr, is_escaped, position)
            }
            Node::MatchExpr(head, arms, position) => {
                MatchExprAnalyzer::analyze(self, head, arms, position)
            }
            Node::RustExpr(exprs, position) => RustExprAnalyzer::analyze(self, exprs, position),
            Node::Component(name, parameters, body, position) => {
                ComponentAnalyzer::analyze(self, name, parameters, body, position)
            }
            Node::ChildContent => ChildContentAnalyzer::analyze(self),
            Node::FnDirective(_, _, _, _) => (),
            Node::Raw(_) => (),
            Node::UseDirective(name, path, component, position) => {
                UseDirectiveAnalyzer::analyze(self, name, path, component, position)
            }
            Node::ContinueDirective => (),
            Node::BreakDirective => (),
        }
    }

    pub fn run(
        template_path: String,
        node: &Node,
        diagnostic: Diagnostic,
        struct_fields: Vec<String>,
        no_warn: bool,
    ) -> Self {
        let mut analyzer = Self::new(diagnostic, struct_fields, no_warn);

        analyzer.analyze(node);

        if let Some(layout) = analyzer.layout.clone() {
            analyzer.files.push((template_path, Position::default()));
            analyzer.analyze(&layout);
        }

        UseDirectiveAnalyzer::analyze_uses(&analyzer);

        analyzer
    }

    pub fn diagnostic(
        &self,
        position: &Position,
        title: &str,
        lines: &[usize],
        info: &str,
        name_len: usize,
        level: Level,
    ) {
        let file = self
            .files
            .last()
            .map(|x| x.0.as_str())
            .unwrap_or("<unknown>");

        let message = match level {
            Level::Warning => self
                .diagnostic
                .warning(file, position, title, lines, info, name_len),
            Level::Caution => self
                .diagnostic
                .caution(file, position, title, lines, info, name_len),
        };

        eprintln!("{message}");
    }

    fn get_struct_field(&self, expr: &str) -> Option<String> {
        let rest = expr
            .trim()
            .trim_start_matches('&')
            .trim()
            .strip_prefix("self.")?;

        let candidate = rest.split('.').next().unwrap_or(rest);

        if parse_str::<Member>(candidate).is_ok() {
            Some(candidate.to_string())
        } else {
            None
        }
    }
}
