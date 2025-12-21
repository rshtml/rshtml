use crate::{
    analyzer::{Analyzer, Component, use_directive::UseDirectiveAnalyzer},
    node::Node,
    position::Position,
};
use std::{mem, path::Path};

pub struct TemplateAnalyzer;

impl TemplateAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        path: &Path,
        _name: &str,
        _fn_names: &Vec<String>,
        nodes: &Vec<Node>,
        position: &Position,
    ) {
        analyzer.files.push((path.to_owned(), position.clone()));
        let previous_component =
            mem::replace(&mut analyzer.component, Component::new(path.to_owned()));

        for node in nodes {
            analyzer.analyze(node)
        }

        analyzer
            .components
            .entry(analyzer.component.path.clone())
            .or_insert(analyzer.component.clone());
        UseDirectiveAnalyzer::analyze_uses(analyzer);

        analyzer.component = previous_component;
        analyzer.files.pop();
    }
}
