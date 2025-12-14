use crate::traits::Render;
use std::fmt;
use std::ops::Deref;

pub struct F<T>(pub T);

impl F<fmt::Result> {
    pub fn render(&self, _f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        self.0
    }
}

impl F<&fmt::Result> {
    pub fn render(&self, _f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        *self.0
    }
}

impl<T> F<F<T>>
where
    T: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    pub fn render(&self, f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        (self.0.0)(f)
    }
}

impl<T: fmt::Display> Render for F<T> {
    fn render(&self, f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Deref for F<T> {
    type Target = ();
    fn deref(&self) -> &Self::Target {
        &()
    }
}

impl Render for () {
    fn render(&self, _f: &mut dyn fmt::Write, e: &'static str) -> fmt::Result {
        let magenta = "\x1b[1;35m";
        let reset = "\x1b[0m";

        eprintln!("{magenta}caution:{reset} {e}");
        Err(fmt::Error)
    }
}

impl<T> fmt::Display for F<T>
where
    T: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}
