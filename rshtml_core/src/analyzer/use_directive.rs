use crate::{analyzer::Analyzer, node::Node, position::Position};
use std::path::PathBuf;

pub struct UseDirectiveAnalyzer;

impl UseDirectiveAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        name: &String,
        path: &PathBuf,
        component: &Node,
        position: &Position,
    ) -> Result<(), Vec<String>> {
        if !analyzer.no_warn && analyzer.use_directives.iter().any(|(n, _, _)| n == name) {
            analyzer.warning(
                position,
                &format!("attempt to reuse use directive `{name}`"),
                &[],
                &format!("use directive `{name}` is redefined"),
                "use".len(),
            );
        }

        analyzer
            .use_directives
            .push((name.to_owned(), path.to_owned(), position.to_owned()));

        analyzer
            .components
            .entry(name.to_owned())
            .or_insert((false, false));

        let previous_is_component = analyzer.is_component.clone();
        analyzer.is_component = Some(name.to_owned());

        analyzer.analyze(component)?;

        analyzer.is_component = previous_is_component;

        Ok(())
    }

    pub fn analyze_uses(analyzer: &Analyzer) {
        if analyzer.no_warn {
            return;
        }

        analyzer
            .use_directives
            .iter()
            .filter(|(name, _, _)| !analyzer.components.get(name).is_some_and(|(_, used)| *used))
            .for_each(|(name, _, position)| {
                analyzer.warning(
                    position,
                    &format!("unused use directive `{name}`"),
                    &[],
                    &format!("the use directive `{name}` defined but not used"),
                    "use".len(),
                );
            });
    }
}
