pub struct EscapingWriter<'a, T: ::std::fmt::Write + ?Sized> {
    pub inner: &'a mut T,
}

impl<'a, T: ::std::fmt::Write + ?Sized> ::std::fmt::Write for EscapingWriter<'a, T> {
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
