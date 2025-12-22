use crate::{
    analyzer::{Analyzer, Component, use_directive::UseDirectiveAnalyzer},
    node::{Function, Node},
    position::Position,
};
use std::{mem, path::Path};

pub struct TemplateAnalyzer;

impl TemplateAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        path: &Path,
        _name: &str,
        fns: &Vec<Function>,
        nodes: &Vec<Node>,
        position: &Position,
    ) {
        analyzer.files.push((path.to_owned(), position.clone()));
        let prev_component = mem::replace(
            &mut analyzer.component,
            Component::new(path.to_owned(), fns.to_owned()),
        );

        for node in nodes {
            analyzer.analyze(node)
        }

        let component = mem::replace(&mut analyzer.component, prev_component);

        UseDirectiveAnalyzer::analyze_uses(analyzer, &component);
        analyzer
            .components
            .entry(component.path.to_owned())
            .or_insert(component);

        analyzer.files.pop();
    }
}
