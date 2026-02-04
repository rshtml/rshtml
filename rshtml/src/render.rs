use crate::Write;
use std::fmt;

pub trait Render {
    fn render_e(&self, out: &mut dyn Write, e: &'static str) -> fmt::Result;
}

impl Render for () {
    fn render_e(&self, _out: &mut dyn Write, e: &'static str) -> fmt::Result {
        eprintln!("{e}");
        Err(fmt::Error)
    }
}
