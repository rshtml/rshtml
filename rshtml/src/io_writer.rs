use std::{fmt, io};

pub struct IoWriter<W> {
    writer: W,
    error: Option<io::Error>,
}

impl<W: io::Write> IoWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            error: None,
        }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }

    pub fn error(&self) -> Option<&io::Error> {
        self.error.as_ref()
    }

    pub fn take_error(&mut self) -> Option<io::Error> {
        self.error.take()
    }
}

impl<W: io::Write> fmt::Write for IoWriter<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if let Err(e) = self.writer.write_all(s.as_bytes()) {
            self.error = Some(e);
            return Err(fmt::Error);
        }
        Ok(())
    }
}

impl<W: io::Write> crate::Write for IoWriter<W> {
    fn raw(&mut self) -> &mut dyn crate::Write {
        self
    }
}
