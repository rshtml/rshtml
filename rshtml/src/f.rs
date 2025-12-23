use crate::EscapingWriter;
use crate::traits::Render;
use std::fmt::{self, Debug, Write};
use std::ops::Deref;

#[repr(transparent)]
#[derive(Debug)]
pub struct F<T: ?Sized, const ESCAPE: bool = true>(pub T);

impl<const ESCAPE: bool> F<fmt::Result, ESCAPE> {
    #[inline(always)]
    pub fn render(&self, _f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        self.0
    }
}

impl<const ESCAPE: bool> F<&fmt::Result, ESCAPE> {
    #[inline(always)]
    pub fn render(&self, _f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        *self.0
    }
}

impl<T: ?Sized, const ESCAPE1: bool, const ESCAPE2: bool> F<F<T, ESCAPE1>, ESCAPE2>
where
    T: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    #[inline(always)]
    pub fn render(&self, f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        (self.0.0)(f)
    }
}

impl<T: ?Sized, const ESCAPE1: bool, const ESCAPE2: bool> F<&F<T, ESCAPE1>, ESCAPE2>
where
    T: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    #[inline(always)]
    pub fn render(&self, f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        (self.0.0)(f)
    }
}

impl<T: fmt::Display + ?Sized, const ESCAPE: bool> Render for F<T, ESCAPE> {
    #[inline(always)]
    fn render(&self, f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        if ESCAPE {
            write!(&mut EscapingWriter { inner: f }, "{}", &self.0)
        } else {
            write!(f, "{}", &self.0)
        }
    }
}

impl<T: ?Sized, const ESCAPE: bool> Deref for F<T, ESCAPE> {
    type Target = ();

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &()
    }
}

impl Render for () {
    fn render(&self, _f: &mut dyn fmt::Write, e: &'static str) -> fmt::Result {
        eprintln!("{e}");
        Err(fmt::Error)
    }
}

impl<T: ?Sized, const ESCAPE: bool> fmt::Display for F<T, ESCAPE>
where
    T: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}
