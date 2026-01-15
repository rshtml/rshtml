use crate::{EscapingWriter, ViewIter};
use std::{
    borrow::Cow,
    cell::RefCell,
    fmt::{self, Write},
};

pub trait RsHtml {
    fn fmt(&self, __f__: &mut dyn fmt::Write) -> fmt::Result;
    fn render(&self) -> Result<String, fmt::Error>;
}

pub trait Render {
    fn render(&self, out: &mut dyn fmt::Write, e: &'static str) -> fmt::Result;
}

pub trait View {
    fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result;

    fn text_size(&self) -> usize {
        0
    }
}

impl<T: View + ?Sized> View for &T {
    fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        (*self).render(out)
    }

    fn text_size(&self) -> usize {
        (*self).text_size()
    }
}

impl<T: View> View for [T] {
    fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        for item in self {
            item.render(out)?;
        }
        Ok(())
    }
}

impl<T: View> View for Vec<T> {
    fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        for item in self {
            item.render(out)?;
        }
        Ok(())
    }
}

impl<T: View + ?Sized> View for Box<T> {
    fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        (**self).render(out)
    }
}

impl View for () {
    fn render(&self, _out: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }
}

impl<'a> View for Cow<'a, str> {
    fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        (**self).render(out)
    }
    fn text_size(&self) -> usize {
        self.len()
    }
}

impl<'a> View for fmt::Arguments<'a> {
    fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        write!(&mut EscapingWriter { inner: out }, "{}", self)
    }
}

macro_rules! impl_view_for_display {
    ($($t:ty),*) => {
        $(
            impl View for $t {
                fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
                    write!(&mut EscapingWriter { inner: out }, "{}", self)
                }
            }
        )*
    };
}

impl_view_for_display!(
    String, str, char, bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32,
    f64
);

pub trait IntoViewIter: IntoIterator + Sized {
    fn view_iter(self) -> ViewIter<Self::IntoIter>
    where
        Self::Item: View,
    {
        ViewIter(RefCell::new(Some(self.into_iter())))
    }
}

impl<T: IntoIterator> IntoViewIter for T {}
