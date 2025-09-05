use std::fmt::Write;

pub trait RsHtml {
    fn fmt(&mut self, __f__: &mut dyn Write) -> std::fmt::Result;
    fn render(&mut self) -> Result<String, std::fmt::Error>;
}
