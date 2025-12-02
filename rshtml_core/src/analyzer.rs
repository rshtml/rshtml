mod child_content;
mod component;
mod extends_directive;
mod match_expr;
mod render_directive;
mod rust_block;
mod rust_expr;
mod rust_expr_simple;
mod section_block;
mod section_directive;
mod template;
mod use_directive;

use crate::{
    analyzer::{
        child_content::ChildContentAnalyzer, component::ComponentAnalyzer,
        extends_directive::ExtendsDirectiveAnalyzer, match_expr::MatchExprAnalyzer,
        render_directive::RenderDirectiveAnalyzer, rust_block::RustBlockAnalyzer,
        rust_expr::RustExprAnalyzer, rust_expr_simple::RustExprSimpleAnalyzer,
        section_block::SectionBlockAnalyzer, section_directive::SectionDirectiveAnalyzer,
        template::TemplateAnalyzer, use_directive::UseDirectiveAnalyzer,
    },
    node::Node,
    position::Position,
};
use std::{collections::HashMap, path::PathBuf};

pub struct Analyzer {
    pub files: Vec<(String, Position)>,
    components: HashMap<String, Component>,
    layout_directive: PathBuf,
    pub layout: Option<Node>,
    sources: HashMap<String, String>,
    sections: HashMap<String, Position>,
    pub no_warn: bool,
    is_component: Option<String>,
    render_directives: Vec<String>,
}

impl Analyzer {
    fn new(sources: HashMap<String, String>, no_warn: bool) -> Self {
        Self {
            files: Vec::new(),
            components: HashMap::new(),
            layout_directive: PathBuf::new(),
            layout: None,
            sources,
            sections: HashMap::new(),
            no_warn,
            is_component: None,
            render_directives: Vec::new(),
        }
    }

    fn analyze(&mut self, node: &Node) -> Result<(), Vec<String>> {
        match node {
            Node::Template(file, nodes, position) => {
                TemplateAnalyzer::analyze(self, file, nodes, position)
            }
            Node::Text(_) => Ok(()),
            Node::InnerText(_) => Ok(()),
            Node::Comment(_) => Ok(()),
            Node::ExtendsDirective(path, layout) => {
                ExtendsDirectiveAnalyzer::analyze(self, path, layout)
            }
            Node::RenderDirective(name) => RenderDirectiveAnalyzer::analyze(self, name),
            Node::RustBlock(content, position) => {
                RustBlockAnalyzer::analyze(self, content, position)
            }
            Node::RustExprSimple(expr, is_escaped, position) => {
                RustExprSimpleAnalyzer::analyze(self, expr, is_escaped, position)
            }
            Node::RustExprParen(_, _, _) => Ok(()),
            Node::MatchExpr(head, arms, position) => {
                MatchExprAnalyzer::analyze(self, head, arms, position)
            }
            Node::RustExpr(exprs, position) => RustExprAnalyzer::analyze(self, exprs, position),
            Node::SectionDirective(name, content, position) => {
                SectionDirectiveAnalyzer::analyze(self, name, content, position)
            }
            Node::SectionBlock(name, content) => SectionBlockAnalyzer::analyze(self, name, content),
            Node::RenderBody => Ok(()),
            Node::Component(name, parameters, body, position) => {
                ComponentAnalyzer::analyze(self, name, parameters, body, position)
            }
            Node::ChildContent => ChildContentAnalyzer::analyze(self),
            Node::Raw(_) => Ok(()),
            Node::UseDirective(name, path, component) => {
                UseDirectiveAnalyzer::analyze(self, name, path, component)
            }
            Node::ContinueDirective => Ok(()),
            Node::BreakDirective => Ok(()),
        }
    }

    pub fn run(
        template_path: String,
        node: &Node,
        sources: HashMap<String, String>,
        no_warn: bool,
    ) -> anyhow::Result<()> {
        let mut analyzer = Self::new(sources, no_warn);
        let mut errs = Vec::new();

        if let Err(e) = analyzer.analyze(&node) {
            errs.extend(e);
        }

        if let Some(layout) = analyzer.layout.clone() {
            analyzer.files.push((template_path, Position::default()));
            if let Err(e) = analyzer.analyze(&layout) {
                errs.extend(e);
            }
        }

        RenderDirectiveAnalyzer::analyze_renders(&analyzer);

        errs.is_empty()
            .then(|| ())
            .ok_or(anyhow::anyhow!(errs.join("\n\n")))
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

    fn files_to_info(&self, position: &Position) -> String {
        let positions = self
            .files
            .iter()
            .skip(1)
            .map(|(_, pos)| pos)
            .chain(std::iter::once(position));

        let mappings: Vec<String> = self
            .files
            .iter()
            .zip(positions)
            .map(|((file, _), pos)| pos.as_info(file))
            .collect();

        mappings.join(" > ")
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
}

#[derive(Clone)]
struct Component {
    parameters: Vec<String>,
    code_block_vars: Vec<String>,
    has_child_content: bool,
}

impl Component {
    fn new() -> Self {
        Self {
            parameters: Vec::new(),
            code_block_vars: Vec::new(),
            has_child_content: false,
        }
    }
}
