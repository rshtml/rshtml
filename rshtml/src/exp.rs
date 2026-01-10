use crate::{EscapingWriter, traits::View};
use std::fmt::Write;
use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct Exp<T: ?Sized>(pub T);

impl<T: View + ?Sized> Exp<T> {
    pub fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        self.0.render(out)
    }
}

impl<T: fmt::Display + ?Sized> View for Exp<T> {
    fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        write!(&mut EscapingWriter { inner: out }, "{}", &self.0)
    }
}

impl<T: ?Sized> fmt::Display for Exp<T>
where
    T: View,
{
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0).render(out)
    }
}

impl Exp<()> {
    pub fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        write!(out, "")
    }
}

impl Exp<&()> {
    pub fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        write!(out, "")
    }
}
