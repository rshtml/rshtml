#![cfg_attr(not(debug_assertions), allow(unused))]

use std::{mem, ops::Range};
use winnow::{
    LocatingSlice, Parser,
    ascii::multispace0,
    combinator::{alt, peek, repeat},
    token::{none_of, take_till},
};

#[derive(Debug, Clone)]
pub enum Snippet {
    ExprSimple(Range<usize>, String),
    ExprParen(Range<usize>, String),
    Block(Range<usize>, String),
    Stmt(Range<usize>, String),
    Component(Range<usize>, String),
    ChildContent(Range<usize>, String),
    Continue(Range<usize>, String),
    Break(Range<usize>, String),
    None,
}

impl Snippet {
    pub fn start(&self) -> usize {
        match self {
            Snippet::ExprSimple(r, _)
            | Snippet::ExprParen(r, _)
            | Snippet::Block(r, _)
            | Snippet::Stmt(r, _)
            | Snippet::Component(r, _)
            | Snippet::ChildContent(r, _)
            | Snippet::Continue(r, _)
            | Snippet::Break(r, _) => r.start,

            Snippet::None => 0,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Debug {
    pub snippets: Vec<Snippet>,
    pub source: Vec<u8>,
}

impl Debug {
    pub fn new(source: Vec<u8>) -> Self {
        Self {
            source,
            snippets: Vec::new(),
        }
    }

    pub fn extract(&mut self) {
        self.snippets
            .sort_unstable_by_key(|b| std::cmp::Reverse(b.start()));

        let snippets = mem::take(&mut self.snippets);

        for snippet in snippets {
            if let Snippet::ExprSimple(range, expr) = snippet {
                self.source_replace_insert(expr.as_str(), range.start - 1);
            }
        }
    }

    pub fn push(&mut self, snippet: Snippet) {
        match snippet {
            Snippet::ExprSimple(range, expr) => {
                let expr = expr.strip_prefix('#').unwrap_or(&expr);
                let expr = format!("&{};", expr);
                self.snippets.push(Snippet::ExprSimple(range, expr));
            }
            Snippet::ExprParen(range, expr) => {
                let mut expr = expr.trim();
                expr = expr.strip_prefix('#').unwrap_or(expr);
                expr = expr.strip_prefix('(').unwrap_or(expr);
                expr = expr.strip_suffix(')').unwrap_or(expr);
                let expr = format!("&{};", expr);
                self.source_replace(&expr, range.start - 1);
            }
            Snippet::Block(range, mut block) => {
                block.replace_range(0..1, " ");
                let len = block.len();
                block.replace_range(len - 1..len, " ");
                self.source_replace(&block, range.start);
            }
            Snippet::Stmt(range, stmt) => {
                let ranges =
                    Self::stmt(&mut LocatingSlice::new(stmt.as_str())).unwrap_or(Vec::new());

                let mut stmt_buf = stmt.as_bytes().to_vec();

                for range in ranges {
                    let limit = range.end.min(stmt_buf.len());

                    for item in stmt_buf.iter_mut().take(limit).skip(range.start) {
                        if *item != b'\n' {
                            *item = b' ';
                        }
                    }
                }

                let cleaned_stmt = String::from_utf8(stmt_buf).unwrap();

                self.source_replace(&cleaned_stmt, range.start);
            }
            Snippet::Component(_range, _component) => {}
            Snippet::ChildContent(range, child_content) => {
                let child_content = format!("{};", child_content.trim());
                self.source_replace(&child_content, range.start - 1);
            }
            Snippet::Continue(range, continue_) => {
                let continue_ = format!("{};", continue_.trim());
                self.source_replace(&continue_, range.start - 1);
            }
            Snippet::Break(range, break_) => {
                let break_ = format!("{};", break_.trim());
                self.source_replace(&break_, range.start - 1);
            }
            Snippet::None => {}
        }
    }

    fn stmt(input: &mut LocatingSlice<&str>) -> winnow::Result<Vec<Range<usize>>> {
        take_till(1.., '{').parse_next(input)?;
        let range = Self::skip_braces.parse_next(input)?;

        let mut ranges = repeat(
            0..,
            (
                peek((multispace0, "else")).void(),
                take_till(1.., '{').void(),
                Self::skip_braces,
            )
                .map(|(_, _, r)| r),
        )
        .fold(Vec::new, |mut acc, item| {
            acc.push(item);
            acc
        })
        .parse_next(input)?;

        ranges.insert(0, range);

        Ok(ranges)
    }

    fn skip_braces(input: &mut LocatingSlice<&str>) -> winnow::Result<Range<usize>> {
        (
            "{".void(),
            repeat(
                0..,
                alt((Self::skip_braces.void(), none_of(['{', '}']).void())),
            )
            .fold(|| (), |_, _| ())
            .span(),
            "}".void(),
        )
            .map(|(_, range, _)| range)
            .parse_next(input)
    }

    pub fn source_replace(&mut self, content: &str, start: usize) {
        let content_bytes = content.as_bytes();
        let mut buf_idx = start;

        for &byte in content_bytes {
            if buf_idx < self.source.len() {
                self.source[buf_idx] = byte;
                buf_idx += 1;
            }
        }
    }

    pub fn source_replace_insert(&mut self, content: &str, start: usize) {
        let content_bytes = content.as_bytes();
        let mut buf_idx = start;

        for &byte in content_bytes {
            if self.source[buf_idx] == b'\n' {
                self.source.insert(buf_idx, byte);
                buf_idx += 1;
            } else {
                self.source[buf_idx] = byte;
                buf_idx += 1;
            }
        }
    }
}
