pub struct RenderDirectiveAnalyzer;

use crate::analyzer::Analyzer;

impl RenderDirectiveAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer, name: &str) -> Result<(), Vec<String>> {
        analyzer.render_directives.push(name.to_owned());

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
