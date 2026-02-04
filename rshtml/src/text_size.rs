use crate::View;
use std::ops::Deref;

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
