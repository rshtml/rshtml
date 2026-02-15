use crate::{Render, View, Write};
use std::{
    fmt::{self, Debug, Display},
    ops::Deref,
};

#[derive(Debug)]
pub struct Exp<T: ?Sized>(pub T);

impl<T: View> Exp<T> {
    pub fn render(&self, out: &mut dyn Write) -> fmt::Result {
        self.0.render(out)
    }

    pub fn render_e(&self, out: &mut dyn Write, _e: &'static str) -> fmt::Result {
        self.0.render(out)
    }
}

impl<T: Display> View for Exp<T> {
    fn render(&self, out: &mut dyn Write) -> fmt::Result {
        write!(out, "{}", &self.0)
    }
}

impl<T> fmt::Display for Exp<T>
where
    T: View,
{
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0).render(out)
    }
}

impl<T: Display> Render for Exp<T> {
    fn render_e(&self, out: &mut dyn Write, _e: &'static str) -> fmt::Result {
        write!(out, "{}", &self.0)
    }
}

impl<T> Deref for Exp<T> {
    type Target = ();

    fn deref(&self) -> &Self::Target {
        &()
    }
}
