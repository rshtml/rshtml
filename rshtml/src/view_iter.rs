use crate::traits::View;
use std::{cell::RefCell, fmt};

pub struct ViewIter<I>(pub RefCell<Option<I>>);

impl<I, V> View for ViewIter<I>
where
    I: Iterator<Item = V>,
    V: View,
{
    fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        if let Some(iter) = self.0.borrow_mut().take() {
            for item in iter {
                item.render(out)?;
            }
        }
        Ok(())
    }
}
