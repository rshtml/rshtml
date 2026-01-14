use crate::traits::View;
use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct Exp<T: ?Sized>(pub T);

impl<T: View + ?Sized> Exp<T> {
    pub fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        self.0.render(out)
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

// impl<I> View for Exp<RefCell<Option<I>>>
// where
//     I: Iterator,
//     I::Item: View,
// {
//     fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
//         if let Some(iter) = self.0.borrow_mut().take() {
//             for item in iter {
//                 item.render(out)?;
//             }
//         }
//         Ok(())
//     }
// }

// pub fn viter<I>(iter: I) -> Exp<RefCell<Option<I::IntoIter>>>
// where
//     I: IntoIterator,
//     I::Item: View,
// {
//     Exp(RefCell::new(Some(iter.into_iter())))
// }
