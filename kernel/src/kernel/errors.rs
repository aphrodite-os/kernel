//! Stuff related to errors.

use crate::display::TextDisplay;

/// An error used by aphrodite
#[derive(Clone, Copy)]
pub struct Error<'a> {
    message: &'a str,
    code: i16,
}

impl<'a> Error<'a> {
    /// Creates a new error.
    pub const fn new(message: &'a str, code: i16) -> Self {
        Error { message, code }
    }
}

impl Error<'_> {
    /// Display the contents of the error on a [TextDisplay] with no prefix.
    pub fn display_np(&self, display: &dyn TextDisplay) {
        crate::output::terrorbnp(&crate::i16_as_u8_slice(self.code), display).unwrap();
        crate::output::terrorsnp(": ", display).unwrap();
        crate::output::terrorsnpln(self.message, display).unwrap();
    }
}

impl core::fmt::Debug for Error<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(&crate::i16_as_u8_slice(self.code)).unwrap())?;
        f.write_str(": ")?;
        f.write_str(self.message)
    }
}

impl core::fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(&crate::i16_as_u8_slice(self.code)).unwrap())?;
        f.write_str(": ")?;
        f.write_str(self.message)
    }
}

impl core::error::Error for Error<'_> {}
