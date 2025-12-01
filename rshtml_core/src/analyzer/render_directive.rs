pub struct RenderDirectiveAnalyzer;

use anyhow::{Result, anyhow};

use crate::{analyzer::Analyzer, position::Position};

impl RenderDirectiveAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer, name: &str, position: &Position) -> Result<()> {
        analyzer.position = position.to_owned();

        if !analyzer.sections.contains_key(name) {
            let message = analyzer.message(
                &format!("attempt to use an undefined section"),
                &[],
                &format!("render was used, but the section `{name}` was not defined."),
                "@render".len(),
            );

            return Err(anyhow!(message));
        }

        Ok(())
    }
}
