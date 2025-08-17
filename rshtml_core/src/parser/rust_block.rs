use crate::Node;
use crate::node::{RustBlockContent, TextLineItem};
use crate::parser::{IParser, RsHtmlParser, Rule};
use crate::traits::IsEscaped;
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
                Rule::text_line_directive => {
                    content_parts.push(Self::build_text_line(inner_pair));
                }
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

    fn build_text_line(inner_pair: Pair<Rule>) -> RustBlockContent {
        let mut items = Vec::new();

        for item_pair in inner_pair.into_inner() {
            match item_pair.as_rule() {
                Rule::rust_expr_simple => {
                    let item_pair_str = item_pair.as_str();
                    items.push(TextLineItem::RustExprSimple(
                        item_pair_str.escaped_or_raw(),
                        item_pair_str.is_escaped(),
                    ));
                }
                Rule::text_line => {
                    let text = item_pair.as_str().replace("@@", "@");
                    if !text.is_empty() {
                        items.push(TextLineItem::Text(text));
                    }
                }
                _ => {
                    eprintln!(
                        "Warning: Unexpected rule in text_block: {:?}",
                        item_pair.as_rule()
                    );
                }
            }
        }

        RustBlockContent::TextLine(items)
    }
}
