use crate::Write;

pub struct EscapingWriter<'a> {
    pub inner: &'a mut dyn Write,
}

impl<'a> ::std::fmt::Write for EscapingWriter<'a> {
    fn write_str(&mut self, input: &str) -> ::std::fmt::Result {
        for c in input.chars() {
            match c {
                '&' => self.inner.write_str("&amp;")?,
                '<' => self.inner.write_str("&lt;")?,
                '>' => self.inner.write_str("&gt;")?,
                '"' => self.inner.write_str("&quot;")?,
                '\'' => self.inner.write_str("&#39;")?,
                '/' => self.inner.write_str("&#x2F;")?,
                _ => self.inner.write_char(c)?,
            }
        }

        Ok(())
    }
}

impl<'a> Write for EscapingWriter<'a> {
    fn raw(&mut self) -> &mut dyn Write {
        self.inner
    }
}
