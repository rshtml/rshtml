use std::{ffi::OsString, fmt};

pub trait Write: fmt::Write {
    fn raw(&mut self) -> &mut dyn Write;
}

impl Write for String {
    fn raw(&mut self) -> &mut dyn Write {
        self
    }
}

impl Write for fmt::Formatter<'_> {
    fn raw(&mut self) -> &mut dyn Write {
        self
    }
}

impl Write for OsString {
    fn raw(&mut self) -> &mut dyn Write {
        self
    }
}

impl<W: Write> Write for &mut W {
    fn raw(&mut self) -> &mut dyn Write {
        (**self).raw()
    }
}
