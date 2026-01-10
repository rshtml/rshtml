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
}

impl<T: View + ?Sized> View for &T {
    fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        (*self).render(out)
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

// impl<F> View for F
// where
//     out: Fn(&mut dyn fmt::Write) -> fmt::Result,
// {
//     fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
//         self(f)
//     }
// }

// impl<'a, I, V> View for I
// where
//     I: IntoIterator<Item = V> + Clone + 'a,
//     V: View + 'a,
// {
//     fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
//         for item in self.clone() {
//             item.render(f)?;
//         }
//         Ok(())
//     }
// }

// impl<'a, I> View for &'a I
// where
//     &'a I: IntoIterator,
//     <&'a I as IntoIterator>::Item: View,
// {
//     fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
//         for item in *self {
//             item.render(f)?;
//         }
//         Ok(())
//     }
// }
