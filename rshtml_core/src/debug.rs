#![cfg_attr(not(debug_assertions), allow(unused))]

use crate::context::Op;
use std::ops::Range;

#[derive(Debug, Default, Clone)]
pub struct Debug {
    pub cleaned_source: Vec<u8>,
}

impl Debug {
    pub fn new(cleaned_source: Vec<u8>) -> Self {
        Self { cleaned_source }
    }

    pub fn process(&mut self, source: &str, ops: &[Op]) {
        let mut params_range = None;
        let extract = |range: &Range<usize>| &source[range.start..range.end];

        for op in ops.iter().rev() {
            match op {
                Op::ExprSimple(range, _) => {
                    let expr = format!("&{};", extract(range));
                    self.source_replace_insert(expr.as_str(), range.start);
                }
                Op::ExprParen(range, _) => {
                    let expr = format!("&{};", extract(range));
                    self.source_replace(&expr, range.start);
                }
                Op::RustCode(range) => {
                    self.source_replace(extract(range), range.start);
                }
                Op::ChildContent(range) => {
                    let child_content = format!("{};", extract(range));
                    self.source_replace(&child_content, range.start);
                }
                Op::ContinueOrBreak(range) => {
                    let continue_or_break = format!("{};", extract(range));
                    self.source_replace(&continue_or_break, range.start);
                }
                Op::TemplateParams(range) => {
                    self.source_replace(extract(range), range.start);
                    params_range = Some(range);
                }
                Op::Text(range) => {
                    let text = extract(range);
                    let mut bytes_to_delete = text.len() - text.chars().count();
                    if bytes_to_delete > 0 {
                        let mut r = range.end;
                        let mut w = range.end;

                        while r > range.start {
                            r -= 1;
                            let b = self.cleaned_source[r];
                            if bytes_to_delete > 0 && b == b' ' {
                                bytes_to_delete -= 1;
                                continue;
                            }
                            w -= 1;
                            self.cleaned_source[w] = b;
                        }

                        self.cleaned_source.drain(range.start..w);
                    }
                }
                Op::OpenBrace(range) => self.source_replace(extract(range), range.start),
                Op::CloseBrace(range) => self.source_replace(extract(range), range.start),
                Op::ComponentAttribute(_, Some(range)) => {
                    let expr = format!("&{};", extract(range));
                    self.source_replace(&expr, range.start);
                }
                _ => {}
            }
        }

        if let Some(range) = params_range {
            let mut out = Vec::with_capacity(self.cleaned_source.len() + 96);

            out.extend_from_slice(b"fn render(child_content: impl ::rshtml::View,");
            out.extend_from_slice(&self.cleaned_source[..range.start]);
            out.push(b' ');
            out.extend_from_slice(&self.cleaned_source[range.start + 1..range.end]);
            out.extend_from_slice(b"->::std::fmt::Result{");
            out.extend_from_slice(&self.cleaned_source[range.end..]);

            self.cleaned_source = out;
        } else {
            let mut out = Vec::with_capacity(self.cleaned_source.len() + 64);
            out.extend_from_slice(
                b"fn render(child_content: impl ::rshtml::View)->::std::fmt::Result{",
            );
            out.extend_from_slice(&self.cleaned_source);
            self.cleaned_source = out;
        }
    }

    pub fn source_replace(&mut self, content: &str, start: usize) {
        let bytes = content.as_bytes();
        let end = (start + bytes.len()).min(self.cleaned_source.len());
        if start < end {
            self.cleaned_source[start..end].copy_from_slice(&bytes[..end - start]);
        }
    }

    pub fn source_replace_insert(&mut self, content: &str, start: usize) {
        for (buf_idx, &byte) in (start..).zip(content.as_bytes()) {
            match self.cleaned_source.get(buf_idx) {
                Some(b'\n') => self.cleaned_source.insert(buf_idx, byte),
                Some(_) => self.cleaned_source[buf_idx] = byte,
                None => self.cleaned_source.push(byte),
            }
        }
    }
}
