pub(crate) trait IsEscaped {
    fn is_escaped(&self) -> bool;
    fn escaped_or_raw(&self) -> String;
}

impl<T: AsRef<str>> IsEscaped for T {
    fn is_escaped(&self) -> bool {
        !self.as_ref().starts_with("#")
    }

    fn escaped_or_raw(&self) -> String {
        if self.is_escaped() {
            return self.as_ref().to_string();
        }

        self.as_ref().chars().skip(1).collect()
    }
}
