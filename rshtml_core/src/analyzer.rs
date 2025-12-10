mod child_content;
mod component;
mod match_expr;
mod rust_expr;
mod rust_expr_paren;
mod rust_expr_simple;
mod template;
mod use_directive;

use syn::{Member, parse_str};

use crate::{
    analyzer::{
        child_content::ChildContentAnalyzer, component::ComponentAnalyzer,
        match_expr::MatchExprAnalyzer, rust_expr::RustExprAnalyzer,
        rust_expr_paren::RustExprParenAnalyzer, rust_expr_simple::RustExprSimpleAnalyzer,
        template::TemplateAnalyzer, use_directive::UseDirectiveAnalyzer,
    },
    node::Node,
    position::Position,
};
use std::{collections::HashMap, path::PathBuf};

pub struct Analyzer {
    files: Vec<(String, Position)>,
    use_directives: Vec<(String, PathBuf, Position)>,
    components: HashMap<String, (bool, bool)>, // has_child_content, is_used
    layout: Option<Node>,
    sources: HashMap<String, String>,
    no_warn: bool,
    is_component: Option<String>,
    struct_fields: Vec<String>,
}

impl Analyzer {
    fn new(sources: HashMap<String, String>, struct_fields: Vec<String>, no_warn: bool) -> Self {
        Self {
            files: Vec::new(),
            use_directives: Vec::new(),
            components: HashMap::new(),
            layout: None,
            sources,
            no_warn,
            is_component: None,
            struct_fields,
        }
    }

    fn analyze(&mut self, node: &Node) {
        match node {
            Node::Template(file, name, nodes, position) => {
                TemplateAnalyzer::analyze(self, file, name, nodes, position)
            }
            Node::Text(_) => (),
            Node::InnerText(_) => (),
            Node::Comment(_) => (),
            Node::PropsDirective(_, _) => (),
            Node::RustBlock(_, _) => (),
            Node::RustExprSimple(expr, is_escaped, position) => {
                RustExprSimpleAnalyzer::analyze(self, expr, is_escaped, position)
            }
            Node::RustExprParen(expr, is_escaped, position) => {
                RustExprParenAnalyzer::analyze(self, expr, is_escaped, position)
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
        sources: HashMap<String, String>,
        struct_fields: Vec<String>,
        no_warn: bool,
    ) {
        let mut analyzer = Self::new(sources, struct_fields, no_warn);

        analyzer.analyze(node);

        if let Some(layout) = analyzer.layout.clone() {
            analyzer.files.push((template_path, Position::default()));
            analyzer.analyze(&layout);
        }

        UseDirectiveAnalyzer::analyze_uses(&analyzer);
    }

    pub fn message(
        &self,
        position: &Position,
        title: &str,
        lines: &[usize],
        info: &str,
        name_len: usize,
    ) -> String {
        let lines = if lines.is_empty() {
            &[(position.0).0]
        } else {
            lines
        };

        let (source_snippet, left_pad) = if lines.is_empty() {
            (
                self.source_first_line(position).unwrap_or_default(),
                ((position.0).0).to_string().len(),
            )
        } else {
            (
                self.extract_source_snippet(position).unwrap_or_default(),
                ((position.1).0).to_string().len(),
            )
        };

        let lp = " ".repeat(left_pad);
        let file_info = self.files_to_info(position);
        let info = if info.is_empty() {
            "".to_string()
        } else {
            let hyphen = "-".repeat(name_len);
            format!("{lp} | {hyphen} {info}\n")
        };

        let mut source = String::new();

        let first_line = (position.0).0;

        for (i, source_line) in source_snippet.lines().enumerate() {
            let current_line = first_line + i;
            let lp = left_pad - current_line.to_string().len();
            let lp = " ".repeat(lp);

            if lines.contains(&current_line) {
                source.push_str(format!("{lp}{current_line} | {source_line}\n").as_str());
            }
        }

        let title = if !title.is_empty() {
            format!("{title}\n")
        } else {
            "".to_string()
        };

        let lp = " ".repeat(left_pad);
        let warn = format!("{title}{lp} --> {file_info}\n{lp} |\n{source}{info}{lp} |",);

        warn
    }

    pub fn warning(
        &self,
        position: &Position,
        title: &str,
        lines: &[usize],
        info: &str,
        name_len: usize,
    ) {
        let yellow = "\x1b[33m";
        let reset = "\x1b[0m";

        let warn = self.message(position, title, lines, info, name_len);

        eprintln!("{yellow}warning:{reset} {warn}");
    }

    pub fn caution(
        &self,
        position: &Position,
        title: &str,
        lines: &[usize],
        info: &str,
        name_len: usize,
    ) {
        let magenta = "\x1b[1;35m";
        let reset = "\x1b[0m";

        let cau = self.message(position, title, lines, info, name_len);

        eprintln!("{magenta}caution:{reset} {cau}");
    }

    fn files_to_info(&self, position: &Position) -> String {
        self.files
            .last()
            .map(|(file, _)| position.as_info(file))
            .unwrap_or("<unknown>".to_string())
    }

    fn extract_source_snippet(&self, position: &Position) -> Option<String> {
        let file_path = self.files.last().map(|x| x.0.clone())?;

        let snippet = self
            .sources
            .get(&file_path)?
            .lines()
            .enumerate()
            .skip((position.0).0.saturating_sub(1))
            .take((position.1).0 - (position.0).0 + 1)
            .map(|(i, line)| {
                let skip_count = (i == (position.0).0 - 1) as usize * ((position.0).1 - 1);
                let take_count = ((i == (position.1).0 - 1) as usize
                    * (((position.1).1 - 1).saturating_sub(skip_count)))
                    + ((1 - (i == (position.1).0 - 1) as usize) * usize::MAX);

                line.chars()
                    .skip(skip_count)
                    .take(take_count)
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        Some(snippet)
    }

    fn source_first_line(&self, position: &Position) -> Option<String> {
        let file_path = self.files.last().map(|x| x.0.clone())?;
        self.sources
            .get(&file_path)?
            .lines()
            .nth((position.0).0.saturating_sub(1))
            .map(|s| s.to_string())
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
