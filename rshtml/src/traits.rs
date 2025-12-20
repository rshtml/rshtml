use std::fmt;

pub trait RsHtml {
    fn fmt(&self, __f__: &mut dyn fmt::Write) -> fmt::Result;
    fn render(&self) -> Result<String, fmt::Error>;
}

pub trait Render {
    fn render(&self, f: &mut dyn fmt::Write, e: &'static str) -> fmt::Result;
}
