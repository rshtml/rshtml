use crate::EscapingWriter;
use crate::traits::Render;
use std::fmt::{self, Debug, Write};
use std::ops::Deref;

#[repr(transparent)]
#[derive(Debug)]
pub struct Expr<T: ?Sized, const ESCAPE: bool = true>(pub T);
pub type Block<T> = Expr<T, false>;

impl<const ESCAPE: bool> Expr<fmt::Result, ESCAPE> {
    #[inline(always)]
    pub fn render(&self, _f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        self.0
    }
}

impl<const ESCAPE: bool> Expr<&fmt::Result, ESCAPE> {
    #[inline(always)]
    pub fn render(&self, _f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        *self.0
    }
}

impl<T: ?Sized, const ESCAPE1: bool, const ESCAPE2: bool> Expr<Expr<T, ESCAPE1>, ESCAPE2>
where
    T: Render,
{
    #[inline(always)]
    pub fn render(&self, f: &mut dyn fmt::Write, e: &'static str) -> fmt::Result {
        (self.0.0).render(f, e)
    }
}

impl<T: ?Sized, const ESCAPE1: bool, const ESCAPE2: bool> Expr<&Expr<T, ESCAPE1>, ESCAPE2>
where
    T: Render,
{
    #[inline(always)]
    pub fn render(&self, f: &mut dyn fmt::Write, e: &'static str) -> fmt::Result {
        (self.0.0).render(f, e)
    }
}

impl<T: fmt::Display + ?Sized, const ESCAPE: bool> Render for Expr<T, ESCAPE> {
    #[inline(always)]
    fn render(&self, f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        if ESCAPE {
            write!(&mut EscapingWriter { inner: f }, "{}", &self.0)
        } else {
            write!(f, "{}", &self.0)
        }
    }
}

impl<T: ?Sized, const ESCAPE: bool> Deref for Expr<T, ESCAPE> {
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

impl<T: ?Sized, const ESCAPE: bool> fmt::Display for Expr<T, ESCAPE>
where
    T: Render,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0).render(f, "")
    }
}

impl<T> Render for T
where
    T: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    fn render(&self, f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        (self)(f)
    }
}
