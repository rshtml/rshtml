use crate::traits::View;
use std::{fmt, ops::Deref};

pub struct ViewFn<T>(pub T);

impl<'a, T> View for ViewFn<T>
where
    T: Fn(&mut dyn fmt::Write) -> fmt::Result + 'a,
{
    fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        (self.0)(f)
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

/*
pub struct ViewIter<I>(pub I);

impl<'a, I, V> View for ViewIter<I>
where
    I: IntoIterator<Item = V> + Clone + 'a,
    I::IntoIter: 'a,
    V: View + 'a,
{
    fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        for item in self.0.clone() {
            item.render(f)?;
        }
        Ok(())
    }
}

// Kısa isim için
pub fn viter<I>(iter: I) -> ViewIter<I> {
    ViewIter(iter)
}
*/

// impl<T> View for V<T>
// where
//     T: Fn(&mut dyn fmt::Write) -> fmt::Result + 'static,
// {
//     fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
//         (self.0)(f)
//     }
// }

// impl<T> V<T>
// where
//     T: Fn(&mut dyn fmt::Write) -> fmt::Result + 'static,
// {
//     pub fn boxed(self) -> Box<dyn View + 'static> {
//         Box::new(self)
//     }
// }

// impl<T> From<V<T>> for Box<dyn View>
// where
//     T: Fn(&mut dyn fmt::Write) -> fmt::Result,
// {
//     fn from(v: V<T>) -> Self
//     where
//         T: 'a,
//     {
//         Box::new(v)
//     }
// }
