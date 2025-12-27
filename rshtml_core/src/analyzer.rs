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
    node::{Function, Node},
    position::Position,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use syn::{Member, parse_str};

pub struct Analyzer {
    files: Vec<(PathBuf, Position)>,
    components: HashMap<PathBuf, Component>,
    component: Component,
    layout: Option<Node>,
    no_warn: bool,
    struct_fields: Vec<String>,
    pub diagnostic: Diagnostic,
}

impl Analyzer {
    fn new(diagnostic: Diagnostic, struct_fields: Vec<String>, no_warn: bool) -> Self {
        Self {
            files: Vec::new(),
            components: HashMap::new(),
            component: Component::default(),
            layout: None,
            diagnostic,
            no_warn,
            struct_fields,
        }
    }

    fn analyze(&mut self, node: &Node) {
        match node {
            Node::Template(path, name, fns, nodes, position) => {
                TemplateAnalyzer::analyze(self, path, name, fns, nodes, position)
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
            analyzer
                .files
                .push((PathBuf::from(template_path), Position::default()));
            analyzer.analyze(&layout);
        }

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
            .map(|x| x.0.as_path())
            .unwrap_or(Path::new("<unknown>"));

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

#[derive(Default, Clone)]
struct Component {
    path: PathBuf,
    has_child_content: bool,
    parameters: Vec<String>,
    use_directives: Vec<UseDirective>,
    fns: Vec<Function>,
}

impl Component {
    pub fn new(path: PathBuf, fns: Vec<Function>) -> Self {
        Self {
            path,
            fns,
            ..Default::default()
        }
    }
}

#[derive(Default, Clone)]
struct UseDirective {
    name: String,
    path: PathBuf,
    position: Position,
    is_used: bool,
}
