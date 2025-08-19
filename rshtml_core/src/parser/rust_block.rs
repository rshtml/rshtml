use crate::Node;
use crate::node::RustBlockContent;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair, Pairs};

pub struct RustBlockParser;

impl IParser for RustBlockParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        Ok(Node::RustBlock(Self::build_rust_block_contents(
            pair.into_inner(),
        )?))
    }
}

impl RustBlockParser {
    fn build_rust_block_contents(
        pairs: Pairs<Rule>,
    ) -> Result<Vec<RustBlockContent>, Box<Error<Rule>>> {
        let mut content_parts = Vec::new();
        for inner_pair in pairs {
            match inner_pair.as_rule() {
                Rule::rust_code => {
                    content_parts.push(RustBlockContent::Code(inner_pair.as_str().to_string()));
                }
                Rule::nested_block => {
                    let nested_contents = Self::build_rust_block_contents(inner_pair.into_inner())?;
                    content_parts.push(RustBlockContent::NestedBlock(nested_contents));
                }
                rule => {
                    return Err(Box::new(Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: format!(
                                "Internal Error: Unexpected rule {rule:?} inside rust block content"
                            ),
                        },
                        inner_pair.as_span(),
                    )));
                }
            }
        }
        Ok(content_parts)
    }
}
