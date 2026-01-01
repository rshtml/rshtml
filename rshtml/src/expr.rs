use crate::traits::Render;
use std::fmt::{self, Debug};
use std::ops::Deref;

#[repr(transparent)]
#[derive(Debug)]
pub struct Expr<T: ?Sized>(pub T);
pub type Block<T> = Expr<T>;

impl Expr<fmt::Result> {
    #[inline(always)]
    pub fn render(&self, _f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        self.0
    }
}

impl Expr<&fmt::Result> {
    #[inline(always)]
    pub fn render(&self, _f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        *self.0
    }
}

impl<T: Render + ?Sized> Expr<T> {
    #[inline(always)]
    pub fn render(&self, f: &mut dyn fmt::Write, e: &'static str) -> fmt::Result {
        self.0.render(f, e)
    }
}

impl<T: fmt::Display + ?Sized> Render for Expr<T> {
    #[inline(always)]
    fn render(&self, f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl<T: fmt::Display + ?Sized> Render for &Expr<T> {
    #[inline(always)]
    fn render(&self, f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl<T: ?Sized> fmt::Display for Expr<T>
where
    T: Render,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0).render(f, "")
    }
}

impl<T: ?Sized> Deref for Expr<T> {
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

impl<T> Render for T
where
    T: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    fn render(&self, f: &mut dyn fmt::Write, _e: &'static str) -> fmt::Result {
        (self)(f)
    }
}

impl<T: Render> Render for Vec<T> {
    fn render(&self, f: &mut dyn fmt::Write, e: &'static str) -> fmt::Result {
        for item in self {
            item.render(f, e)?;
        }
        Ok(())
    }
}

impl<T: Render> Render for &Vec<T> {
    fn render(&self, f: &mut dyn fmt::Write, e: &'static str) -> fmt::Result {
        for item in *self {
            item.render(f, e)?;
        }
        Ok(())
    }
}
