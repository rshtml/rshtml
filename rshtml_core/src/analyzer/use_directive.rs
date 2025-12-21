use crate::{
    analyzer::{Analyzer, Component, UseDirective},
    diagnostic::Level,
    node::Node,
    position::Position,
};
use std::{mem, path::PathBuf};

pub struct UseDirectiveAnalyzer;

impl UseDirectiveAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        name: &String,
        path: &PathBuf,
        component: &Node,
        position: &Position,
    ) {
        if !analyzer.no_warn
            && analyzer
                .component
                .use_directives
                .iter()
                .any(|use_directive| &use_directive.name == name)
        {
            analyzer.diagnostic(
                position,
                &format!("attempt to reuse use directive `{name}`"),
                &[],
                &format!("use directive `{name}` is redefined"),
                "use".len(),
                Level::Warning,
            );
        }

        analyzer.component.use_directives.push(UseDirective {
            name: name.to_owned(),
            path: path.to_owned(),
            position: position.to_owned(),
            is_used: false,
        });

        let previous_component =
            mem::replace(&mut analyzer.component, Component::new(path.to_owned()));

        analyzer.analyze(component);

        analyzer.component = previous_component;
    }

    pub fn analyze_uses(analyzer: &Analyzer) {
        if analyzer.no_warn {
            return;
        }

        analyzer
            .component
            .use_directives
            .iter()
            .filter(|use_directive| !use_directive.is_used)
            .for_each(|use_directive| {
                analyzer.diagnostic(
                    &use_directive.position,
                    &format!("unused use directive `{}`", use_directive.name),
                    &[],
                    &format!(
                        "the use directive `{}` defined but not used",
                        use_directive.name
                    ),
                    "use".len(),
                    Level::Warning,
                );
            });
    }
}
