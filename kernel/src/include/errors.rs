//! Stuff related to errors.

/// An error used by aphrodite
pub struct Error<'a> {
    message: &'a str,
    code: i16
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