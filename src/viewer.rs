// use pest::Parser;
// use pest::iterators::Pairs;
// use pest_derive::Parser;
// 
// #[derive(Parser)]
// #[grammar = "template.pest"]
// pub struct TemplateParser;
// 
// pub fn execute_template(input: &str) {
//     match TemplateParser::parse(Rule::template, input) {
//         Ok(pairs) => {
//             execute_pairs(pairs, 0, true);
//         }
//         Err(e) => println!("Parse Error:\n{}", e),
//     }
// }

use pest::iterators::Pairs;
use crate::parser::Rule;

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
            if start {
                print!("{} - {:?}", "  ".repeat(indent), pair.as_rule());
            } else {
                print!(" > {:?}", pair.as_rule());
            }

            execute_pairs(pair.into_inner(), indent + 1, false);
        }
    }
}
