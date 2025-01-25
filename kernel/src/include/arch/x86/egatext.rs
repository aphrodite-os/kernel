//! Stuff for writing and reading to the EGA text buffer.
#![cfg(any(target_arch = "x86"))]

/// Information about the framebuffer.
#[derive(Clone, Copy)]
pub struct FramebufferInfo {
    /// A pointer to the framebuffer.
    pub address: u64,
    /// The pitch of the framebuffer (i.e. the number of bytes in each row).
    pub pitch: u32,
    /// The width of the framebuffer.
    pub width: u32,
    /// The height of the framebuffer.
    pub height: u32,
    /// Bits per pixel.
    pub bpp: u8,
}

/// Returned when the provided position is invalid in the X direction.
pub const ERR_INVALID_X: i16 = -1;
/// Returned when the provided position is invalid in the Y direction.
pub const ERR_INVALID_Y: i16 = -2;


/// White text on a black background.
pub const WHITE_ON_BLACK: u8 = 0b00000111;
/// Black text on a black background.
pub const BLACK_ON_BLACK: u8 = 0b00000000;

impl FramebufferInfo {
    /// Writes a character to the screen.
    pub fn write_char(self, pos: (u32, u32), char: u8, color: u8) -> Result<(), crate::Error<'static>> {
        if pos.0>self.width {
            return Err(crate::Error::new("Invalid X position", ERR_INVALID_X));
        }
        if pos.1>self.height {
            return Err(crate::Error::new("Invalid Y position", ERR_INVALID_Y));
        }
        unsafe {
            let mut addr = self.address as usize;
            addr += (pos.1*self.pitch) as usize;
            addr += (pos.0*(self.bpp as u32/8)) as usize;
            let base_ptr = addr as *mut u16;
            (*base_ptr) = ((color as u16)<<8) | (char as u16);
        }
        Ok(())
    }

    /// Clears the screen.
    pub fn clear_screen(self, color: u8) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.write_char((x, y), b' ', color).unwrap();
            }
        }
    }

    /// Writes a &str to the screen.
    pub fn write_str(self, pos: (u32, u32), str: &str, color: u8) -> Result<(), crate::Error<'static>> {
        let (mut x, mut y) = pos;
        for char in str.as_bytes() {
            self.write_char((x, y), *char, color)?;
            x += 1;
            if x>self.width {
                x -= self.width;
                y += 1;
            }
        }
        Ok(())
    }
}