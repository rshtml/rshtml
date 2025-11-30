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

        let previous_is_component = analyzer.is_component.clone();
        analyzer.is_component = Some(name.to_owned());

        analyzer.analyze(component)?;

        analyzer.is_component = previous_is_component;

        Ok(())
    }
}
