use crate::traits::View;
use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct Exp<T: ?Sized>(pub T);

impl<T: View + ?Sized> Exp<T> {
    pub fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.0.render(f)
    }
}

impl<T: fmt::Display + ?Sized> View for Exp<T> {
    fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl<T: ?Sized> fmt::Display for Exp<T>
where
    T: View,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0).render(f)
    }
}

impl Exp<()> {
    pub fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "")
    }
}

// impl<'a, I, V> Exp<I>
// where
//     I: IntoIterator<Item = V> + Clone + 'a,
//     V: View + 'a,
// {
//     fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
//         for item in self.0.clone() {
//             item.render(f)?;
//         }
//         Ok(())
//     }
// }
