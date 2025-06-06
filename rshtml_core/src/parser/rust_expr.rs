use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair, Pairs};
use std::iter::Peekable;

pub struct RustExprParser;

impl IParser for RustExprParser {
    fn parse(parser: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let pair_span = pair.as_span();

        let mut inner_pairs = pair.into_inner().peekable();
        let mut clauses: Vec<(String, Vec<Node>)> = Vec::new();

        let consume_whitespaces = |inner_p: &mut Peekable<Pairs<Rule>>| {
            while let Some(p) = inner_p.peek() {
                if p.as_rule() == Rule::WHITESPACE {
                    inner_p.next();
                } else {
                    break;
                }
            }
        };

        while inner_pairs.peek().is_some() {
            let head_pair = inner_pairs.next_if(|p| p.as_rule() == Rule::rust_expr_head).ok_or(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Internal Error: rust_expr expected a head, found {:?}", inner_pairs.peek().map(|p| p.as_rule())),
                },
                pair_span,
            ))?;
            let head = head_pair.as_str().trim().to_string();

            consume_whitespaces(&mut inner_pairs);

            let template_pair = inner_pairs.next_if(|p| p.as_rule() == Rule::inner_template).ok_or(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Internal Error: rust_expr missing inner_template for head: '{}'", head),
                },
                pair_span,
            ))?;

            let body_nodes = parser.build_nodes_from_pairs(template_pair.into_inner())?;

            clauses.push((head.clone(), body_nodes));

            consume_whitespaces(&mut inner_pairs);
        }

        if clauses.is_empty() {
            return Err(Box::new(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: "Internal Error: rust_expr parsed with no clauses".to_string(),
                },
                pair_span,
            )));
        }

        Ok(Node::RustExpr(clauses))
    }
}
