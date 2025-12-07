use std::fmt::Write;

pub trait RsHtml {
    fn fmt(&self, __f__: &mut dyn Write) -> std::fmt::Result;
    fn render(&self) -> Result<String, std::fmt::Error>;
}
