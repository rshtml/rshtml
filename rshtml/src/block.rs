use std::fmt;

pub struct Block<'a>(pub Box<dyn Fn(&mut dyn fmt::Write) -> fmt::Result + 'a>);

impl<'a> fmt::Display for Block<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}

impl<'a, F> From<F> for Block<'a>
where
    F: Fn(&mut dyn fmt::Write) -> fmt::Result + 'a,
{
    fn from(f: F) -> Self {
        Block(Box::new(f))
    }
}
