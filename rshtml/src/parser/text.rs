use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;
use regex::Regex;

pub struct TextParser;

impl TextParser {
    pub fn remove_extra_newlines(text: &str) -> String {
        let text = text.replace("\r\n", "\n");
        //let re = Regex::new(r"\n+").unwrap();
        //re.replace_all(&text, "\n").into_owned()
        let re = Regex::new(r"\n{3,}").unwrap();
        re.replace_all(&text, "\n\n").into_owned()
    }
}

impl IParser for TextParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
        let text = pair.as_str().replace("@@", "@").replace("@@{", "{").replace("@@}", "}");

        let text = Self::remove_extra_newlines(&text);

        Ok(Node::Text(text))
    }
}
