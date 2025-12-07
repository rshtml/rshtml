use std::fmt;

pub struct Block<T>(pub T);

impl<T> fmt::Display for Block<T>
where
    T: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}

impl<T> From<T> for Block<T>
where
    T: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    fn from(f: T) -> Self {
        Block(f)
    }
}
