//! Types, constants and traits for displaying text. Mostly implemented in arch/.

use core::fmt::Write;

/// A type used for color in the functions of [TextDisplay].
/// 
/// Type alias for (u8, bool). Boolean argument is whether to
/// change the value(i.e. for [COLOR_BLACK] and [COLOR_DEFAULT]).
pub type Color = (u8, bool);

/// Black-on-black.
pub const COLOR_BLACK: Color = (0, true);

/// Should be whatever color commonly used for status messages.
/// Generally should be white-on-black. Value is one.
pub const COLOR_DEFAULT: Color = (1, true);

/// Some form of display that can be written to with text.
pub trait TextDisplay: core::fmt::Write {
    /// Writes a single character to the specified position.
    fn write_char(&self, pos: (u32, u32), char: u8, color: Color) -> Result<(), crate::Error<'static>>;
    /// Gets the size of the screen.
    fn get_size(&self) -> (u32, u32);
}

impl dyn TextDisplay + '_ {
    /// Clears the screen.
    pub fn clear_screen(&self, color: Color) {
        let (width, height) = self.get_size();
        for x in 0..width {
            for y in 0..height {
                self.write_char((x, y), b' ', color).unwrap();
            }
        }
    }

    /// Writes a &str to the screen.
    pub fn write_str(&self, pos: (u32, u32), str: &str, color: Color) -> Result<(u32, u32), crate::Error<'static>> {
        let (width, _) = self.get_size();
        let (mut x, mut y) = pos;
        for char in str.as_bytes() {
            self.write_char((x, y), *char, color)?;
            if *char == 0 {
                continue
            }
            x += 1;
            while x>width {
                x -= width;
                y += 1;
            }
        }
        Ok((x, y))
    }

    /// Writes a &\[u8] to the screen.
    pub fn write_bytes(&self, pos: (u32, u32), str: &[u8], color: Color) -> Result<(u32, u32), crate::Error<'static>> {
        let (width, _) = self.get_size();
        let (mut x, mut y) = pos;
        for char in str {
            self.write_char((x, y), *char, color)?;
            if *char == 0 {
                continue
            }
            x += 1;
            while x>width {
                x -= width;
                y += 1;
            }
        }
        Ok((x, y))
    }
}

pub struct NoneTextDisplay {}

impl TextDisplay for NoneTextDisplay {
    fn get_size(&self) -> (u32, u32) {
        (1,1)
    }
    fn write_char(&self, _: (u32, u32), _: u8, _: Color) -> Result<(), crate::Error<'static>> {
        Ok(())
    }
}

impl Write for NoneTextDisplay {
    fn write_char(&mut self, _: char) -> core::fmt::Result {
        Ok(())
    }
    fn write_str(&mut self, _: &str) -> core::fmt::Result {
        Ok(())
    }
}