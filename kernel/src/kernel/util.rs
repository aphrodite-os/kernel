//! Utility functions

/// Converts an i16 to an [u8; 6].
pub const fn i16_as_u8_slice(mut value: i16) -> [u8; 6] {
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
        let digit = value % 10;
        let char = b'0' + digit as u8;
        buf[i] = char;
        value /= 10;
        i -= 1;
    }
    buf
}

/// Converts an u32 to an [u8; 10].
pub const fn u32_as_u8_slice(mut value: u32) -> [u8; 10] {
    let mut buf = [0u8; 10];
    let mut i = 9;
    if value == 0 {
        buf[0] = b'0';
    }
    while value > 0 {
        let digit = value % 10;
        let char = b'0' + digit as u8;
        buf[i] = char;
        value /= 10;
        i -= 1;
    }
    buf
}

/// Converts an u16 to an [u8; 5].
pub const fn u16_as_u8_slice(mut value: u16) -> [u8; 5] {
    let mut buf = [0u8; 5];
    let mut i = 4;
    if value == 0 {
        buf[0] = b'0';
    }
    while value > 0 {
        let digit = value % 10;
        let char = b'0' + digit as u8;
        buf[i] = char;
        value /= 10;
        i -= 1;
    }
    buf
}

/// Converts an u8 to an [u8; 3].
pub const fn u8_as_u8_slice(mut value: u8) -> [u8; 3] {
    let mut buf = [0u8; 3];
    let mut i = 2;
    if value == 0 {
        buf[0] = b'0';
    }
    while value > 0 {
        let digit = value % 10;
        let char = b'0' + digit;
        buf[i] = char;
        value /= 10;
        i -= 1;
    }
    buf
}

/// Converts an usize(32 or 64 bit) to an [u8; 20].
pub const fn usize_as_u8_slice(mut value: usize) -> [u8; 20] {
    let mut buf = [0u8; 20];
    let mut i = 19;
    if value == 0 {
        buf[0] = b'0';
    }
    while value > 0 {
        let digit = value % 10;
        let char = b'0' + digit as u8;
        buf[i] = char;
        value /= 10;
        i -= 1;
    }
    buf
}

/// Converts an u64 to an [u8; 10].
pub const fn u64_as_u8_slice(mut value: u64) -> [u8; 20] {
    let mut buf = [0u8; 20];
    let mut i = 19;
    if value == 0 {
        buf[0] = b'0';
    }
    while value > 0 {
        let digit = value % 10;
        let char = b'0' + digit as u8;
        buf[i] = char;
        value /= 10;
        i -= 1;
    }
    buf
}

/// Converts an &mut \[u8] to a i16.
pub fn str_as_i16(mut value: &[u8]) -> i16 {
    let mut out = 0i16;
    let negative = core::str::from_utf8(value).unwrap().starts_with("-");
    if negative {
        value = &value[1..];
    }
    for byte in value {
        let byte = *byte;
        if !byte.is_ascii_digit() {
            continue;
        }
        out *= 10;
        out += (byte - b'0') as i16;
    }

    let mut reversed = 0;

    while out != 0 {
        reversed = reversed * 10 + out % 10;
        out /= 10;
    }

    reversed
}

/// Converts an &mut \[u8] to a u32.
pub fn str_as_u32(value: &[u8]) -> u32 {
    let mut out = 0u32;
    for byte in value {
        let byte = *byte;
        if !byte.is_ascii_digit() {
            continue;
        }
        out *= 10;
        out += (byte - b'0') as u32;
    }

    let mut reversed = 0;

    while out != 0 {
        reversed = reversed * 10 + out % 10;
        out /= 10;
    }

    reversed
}

/// Converts an &mut \[u8] to a u128.
pub fn str_as_u128(value: &[u8]) -> u128 {
    let mut out = 0u128;
    for byte in value {
        let byte = *byte;
        if !byte.is_ascii_digit() {
            continue;
        }
        out *= 10;
        out += (byte - b'0') as u128;
    }

    let mut reversed = 0;

    while out != 0 {
        reversed = reversed * 10 + out % 10;
        out /= 10;
    }

    reversed
}

/// Converts an &mut \[u8] to a u64.
pub fn str_as_u64(value: &[u8]) -> u64 {
    let mut out = 0u64;
    for byte in value {
        let byte = *byte;
        if !byte.is_ascii_digit() {
            continue;
        }
        out *= 10;
        out += (byte - b'0') as u64;
    }

    let mut reversed = 0;

    while out != 0 {
        reversed = reversed * 10 + out % 10;
        out /= 10;
    }

    reversed
}
