#![cfg_attr(not(debug_assertions), allow(unused))]

use std::{mem, ops::Range};

#[derive(Debug, Clone)]
pub enum Snippet {
    ExprSimple(Range<usize>, String),
    ExprParen(Range<usize>, String),
    Block(Range<usize>, String),
    Stmt(Range<usize>, String),
    ChildContent(Range<usize>, String),
    Continue(Range<usize>, String),
    Break(Range<usize>, String),
    Params(Range<usize>, String),
    Html(Range<usize>, usize),
    None,
}

impl Snippet {
    pub fn start(&self) -> usize {
        match self {
            Snippet::ExprSimple(r, _)
            | Snippet::ExprParen(r, _)
            | Snippet::Block(r, _)
            | Snippet::Stmt(r, _)
            | Snippet::ChildContent(r, _)
            | Snippet::Continue(r, _)
            | Snippet::Break(r, _)
            | Snippet::Params(r, _)
            | Snippet::Html(r, _) => r.start,

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

        self.process();
    }

    pub fn push(&mut self, snippet: Snippet) -> &mut Self {
        self.snippets.push(snippet);
        self
    }

    fn process(&mut self) {
        let snippets = mem::take(&mut self.snippets);
        let mut params_range = None;

        for snippet in snippets {
            match snippet {
                Snippet::ExprSimple(range, expr) => {
                    let hashed = expr.starts_with('#');
                    let expr = expr.strip_prefix('#').unwrap_or(&expr);
                    let expr = format!("&{};", expr);
                    let start = if hashed { range.start } else { range.start - 1 };
                    self.source_replace_insert(expr.as_str(), start);
                }
                Snippet::ExprParen(range, expr) => {
                    let trimmed = expr.trim_start();
                    let lead = expr.len() - trimmed.len();
                    let hashed = trimmed.starts_with('#');

                    let mut expr = expr.trim();
                    expr = expr.strip_prefix('#').unwrap_or(expr);
                    expr = expr.strip_prefix('(').unwrap_or(expr);
                    expr = expr.strip_suffix(')').unwrap_or(expr);
                    let expr = format!("&{};", expr);

                    let start = range.start + lead + usize::from(hashed);
                    self.source_replace(&expr, start);
                }
                Snippet::Block(range, mut block) => {
                    block.replace_range(0..1, " ");
                    let len = block.len();
                    block.replace_range(len - 1..len, " ");
                    self.source_replace(&block, range.start);
                }
                Snippet::Stmt(range, stmt) => {
                    self.source_replace(&stmt, range.start);
                }
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
                Snippet::Params(range, params) => {
                    self.source_replace(&params, range.start);
                    params_range = Some(range);
                }
                Snippet::Html(range, mut bytes_to_delete) => {
                    if bytes_to_delete > 0 {
                        let mut r = range.end;
                        let mut w = range.end;

                        while r > range.start {
                            r -= 1;
                            let b = self.source[r];
                            if bytes_to_delete > 0 && b == b' ' {
                                bytes_to_delete -= 1;
                                continue;
                            }
                            w -= 1;
                            self.source[w] = b;
                        }

                        self.source.drain(range.start..w);
                    }
                }
                Snippet::None => {}
            }
        }

        if let Some(range) = params_range {
            let mut out = Vec::with_capacity(self.source.len() + 96);

            out.extend_from_slice(b"fn render(child_content: impl ::rshtml::View,");
            out.extend_from_slice(&self.source[..range.start]);
            out.push(b' ');
            out.extend_from_slice(&self.source[range.start + 1..range.end]);
            out.extend_from_slice(b"->::std::fmt::Result{");
            out.extend_from_slice(&self.source[range.end..]);

            self.source = out;
        } else {
            let mut out = Vec::with_capacity(self.source.len() + 64);
            out.extend_from_slice(
                b"fn render(child_content: impl ::rshtml::View)->::std::fmt::Result{",
            );
            out.extend_from_slice(&self.source);
            self.source = out;
        }
    }

    pub fn source_replace(&mut self, content: &str, start: usize) {
        let bytes = content.as_bytes();
        let end = (start + bytes.len()).min(self.source.len());
        if start < end {
            self.source[start..end].copy_from_slice(&bytes[..end - start]);
        }
    }

    pub fn source_replace_insert(&mut self, content: &str, start: usize) {
        for (buf_idx, &byte) in (start..).zip(content.as_bytes()) {
            match self.source.get(buf_idx) {
                Some(b'\n') => self.source.insert(buf_idx, byte),
                Some(_) => self.source[buf_idx] = byte,
                None => self.source.push(byte),
            }
        }
    }
}
