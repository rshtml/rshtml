use crate::{
    analyzer::{Analyzer, Component},
    node::Node,
};
use anyhow::Result;
use std::path::PathBuf;

pub struct UseDirectiveAnalyzer;

impl UseDirectiveAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        name: &String,
        _path: &PathBuf,
        component: &Node,
    ) -> Result<()> {
        analyzer
            .components
            .entry(name.to_owned())
            .or_insert(Component::new());

        analyzer.analyze(component)
    }
}
