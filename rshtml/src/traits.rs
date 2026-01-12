use std::fmt;

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
