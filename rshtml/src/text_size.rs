use std::ops::Deref;

use crate::traits::View;

pub struct TextSize<T>(pub T);

impl<T: View> TextSize<T> {
    pub fn text_size(&self) -> usize {
        self.0.text_size()
    }
}

impl<T> Deref for TextSize<T> {
    type Target = ();

    fn deref(&self) -> &Self::Target {
        &()
    }
}
