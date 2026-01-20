use crate::traits::View;
use std::{fmt, ops::Deref};

pub struct ViewFn<T>(pub T, usize);

impl<T> ViewFn<T> {
    pub fn new(c: (T, usize)) -> Self {
        Self(c.0, c.1)
    }
}

impl<T> View for ViewFn<T>
where
    T: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        (self.0)(out)
    }

    fn text_size(&self) -> usize {
        self.1
    }
}

impl<'a, T> ViewFn<T>
where
    T: Fn(&mut dyn fmt::Write) -> fmt::Result + 'a,
{
    pub fn boxed(self) -> Box<dyn View + 'a> {
        Box::new(self)
    }
}

impl<T> Deref for ViewFn<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
