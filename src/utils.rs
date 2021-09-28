pub(crate) struct SliceDisplay<'a, T: 'a>(pub &'a [T]);

impl<'a, T: std::fmt::Display + 'a> std::fmt::Display for SliceDisplay<'a, T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.len() {
            0 => write!(formatter, "[]")?,
            1 => write!(formatter, "[{}]", self.0[0])?,
            2 => write!(formatter, "[{}, {}]", self.0[0], self.0[1])?,
            len if len > 2 => {
                write!(formatter, "[")?;
                for elem in &self.0[..len - 1] {
                    write!(formatter, "{}, ", elem)?;
                }
                write!(formatter, "{}]", self.0[len - 1])?;
            }
            _ => (),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_display() {
        let x = [1, 2, 3];
        let formatted = format!("{}", SliceDisplay(&x));

        let expected = "[1, 2, 3]";
        assert_eq!(formatted, expected);
    }
}
