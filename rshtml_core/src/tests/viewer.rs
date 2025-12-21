use crate::parser::Rule;
use pest::iterators::Pairs;

#[allow(clippy::collapsible_else_if)]
pub fn execute_pairs(pairs: Pairs<Rule>, indent: usize, mut start: bool) {
    let pairs_len = pairs.clone().len();
    for pair in pairs {
        if pair.clone().tokens().len() == 2 {
            if start {
                println!(
                    "{} - {:?}: {:?}",
                    "  ".repeat(indent),
                    pair.as_rule(),
                    pair.as_str()
                );
            } else {
                if pairs_len > 1 {
                    println!(
                        "\n{} - {:?}: {:?}",
                        "  ".repeat(indent),
                        pair.as_rule(),
                        pair.as_str()
                    );
                } else {
                    println!(" > {:?}: {:?}", pair.as_rule(), pair.as_str());
                }
            }

            execute_pairs(pair.into_inner(), indent + 1, true);
            start = true;
        } else {
            if start || pairs_len > 1 {
                print!("{} - {:?}", "  ".repeat(indent), pair.as_rule());
            } else {
                print!(" > {:?}", pair.as_rule());
            }

            execute_pairs(pair.into_inner(), indent + 1, false);
        }
    }
}
