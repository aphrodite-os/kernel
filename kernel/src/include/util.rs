//! Utility functions

/// Converts an i16 to an [u8; 6].
pub fn i16_as_u8_slice(mut value: i16) -> [u8; 6] {
    let mut buf = [0u8; 6];
    let mut i = 0;
    if value < 0 {
        buf[i] = b'-';
        value = -value;
    }
    if value == 0 {
        buf[0] = b'0';
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

/// Converts an u32 to an [u8; 10].
pub fn u32_as_u8_slice(mut value: u32) -> [u8; 10] {
    let mut buf = [0u8; 10];
    let mut i = 9;
    if value == 0 {
        buf[0] = b'0';
    }
    while value > 0 {
        let digit = value%10;
        let char = b'0' + digit as u8;
        buf[i] = char;
        value = value / 10;
        i -= 1;
    }
    buf
}

/// Converts an u8 to an [u8; 3].
pub fn u8_as_u8_slice(mut value: u8) -> [u8; 3] {
    let mut buf = [0u8; 3];
    let mut i = 2;
    if value == 0 {
        buf[0] = b'0';
    }
    while value > 0 {
        let digit = value%10;
        let char = b'0' + digit as u8;
        buf[i] = char;
        value = value / 10;
        i -= 1;
    }
    buf
}

/// Converts an usize(32 or 64 bit) to an [u8; 10].
pub fn usize_as_u8_slice(mut value: usize) -> [u8; 20] {
    let mut buf = [0u8; 20];
    let mut i = 19;
    if value == 0 {
        buf[0] = b'0';
    }
    while value > 0 {
        let digit = value%10;
        let char = b'0' + digit as u8;
        buf[i] = char;
        value = value / 10;
        i -= 1;
    }
    buf
}
