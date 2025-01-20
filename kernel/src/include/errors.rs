//! Stuff related to errors.

/// Converts an i16 to an [u8; 6].
pub fn i16_as_u8_slice(mut value: i16) -> [u8; 6] {
    let mut buf = [0u8; 6];
    let mut i = 0;
    if value < 0 {
        buf[i] = b'-';
        value = -value;
    }
    i = 5;
    while value > 0 {
        let digit = value%10;
        let char = b'0' + digit as u8;
        buf[i] = char;
        value = value / 10;
        i -= 1;
    }
    buf
}

/// An error used by aphrodite
pub struct Error<'a> {
    message: &'a str,
    code: i16
}

impl core::fmt::Debug for Error<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(&i16_as_u8_slice(self.code)).unwrap())?;
        f.write_str(": ")?;
        f.write_str(self.message)
    }
}

impl core::fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(&i16_as_u8_slice(self.code)).unwrap())?;
        f.write_str(": ")?;
        f.write_str(self.message)
    }
}

impl core::error::Error for Error<'_> {}