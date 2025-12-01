pub struct RenderDirectiveAnalyzer;

use crate::{analyzer::Analyzer, position::Position};

impl RenderDirectiveAnalyzer {
    pub fn analyze(
        analyzer: &mut Analyzer,
        name: &str,
        position: &Position,
    ) -> Result<(), Vec<String>> {
        analyzer.render_directives.push(name.to_owned());

        if !analyzer.sections.contains_key(name) {
            let message = analyzer.message(
                position,
                &format!("attempt to use an undefined section"),
                &[],
                &format!("render is used, but the section `{name}` isn't defined."),
                "render".len(),
            );

            return Err(vec![message]);
        }

        Ok(())
    }

    pub fn analyze_renders(analyzer: &Analyzer) {
        if analyzer.no_warn {
            return;
        }

        analyzer
            .sections
            .iter()
            .filter(|x| !analyzer.render_directives.contains(x.0))
            .for_each(|x| {
                let name = x.0;
                let position = x.1;

                analyzer.warning(
                    position,
                    &format!("unused section `{name}`"),
                    &[],
                    &format!("the section `{name}` defined but not used"),
                    "section".len(),
                );
            });
    }
}
