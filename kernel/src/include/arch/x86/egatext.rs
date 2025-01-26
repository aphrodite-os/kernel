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
    /// Whether to change the cursor position after outputting text.
    pub change_cursor: bool,
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
    pub fn write_char(self, mut pos: (u32, u32), char: u8, color: u8) -> Result<(), crate::Error<'static>> {
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
        pos.1 += 1;
        if self.change_cursor {
            self.set_cursor_location(pos);
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
    pub fn write_str(mut self, pos: (u32, u32), str: &str, color: u8) -> Result<(u32, u32), crate::Error<'static>> {
        let (mut x, mut y) = pos;
        let change_cursor = self.change_cursor;
        if change_cursor {
            self.change_cursor = false;
        }
        for char in str.as_bytes() {
            self.write_char((x, y), *char, color)?;
            if *char == 0 {
                continue
            }
            x += 1;
            while x>self.width {
                x -= self.width;
                y += 1;
            }
        }
        if change_cursor {
            self.change_cursor = true;
            self.set_cursor_location((x, y));
        }
        Ok((x, y))
    }

    /// Writes a &\[u8] to the screen.
    pub fn write_bytes(mut self, pos: (u32, u32), str: &[u8], color: u8) -> Result<(u32, u32), crate::Error<'static>> {
        let (mut x, mut y) = pos;
        let change_cursor = self.change_cursor;
        if change_cursor {
            self.change_cursor = false;
        }
        for char in str {
            self.write_char((x, y), *char, color)?;
            if *char == 0 {
                continue
            }
            x += 1;
            while x>self.width {
                x -= self.width;
                y += 1;
            }
        }
        if change_cursor {
            self.change_cursor = true;
            self.set_cursor_location((x, y));
        }
        Ok((x, y))
    }

    /// Disables the cursor.
    pub fn disable_cursor(self) {
        super::ports::outb(0x3D4, 0x0A);
        super::ports::outb(0x3D5, 0x20);
    }

    /// Enables the cursor.
    pub fn enable_cursor(self, start_scan: u8, end_scan: u8) {
        super::ports::outb(0x3D4, 0x0A);
        super::ports::outb(0x3D5, (super::ports::inb(0x3D5) & 0xC0) | start_scan);

        super::ports::outb(0x3D4, 0x0B);
        super::ports::outb(0x3D5, (super::ports::inb(0x3D5) & 0xE0) | end_scan);
    }

    /// Sets the cursor's location.
    pub fn set_cursor_location(self, pos: (u32, u32)) {
        let addr = pos.1 * self.width + pos.0;

        super::ports::outb(0x3D4, 0x0F);
        super::ports::outb(0x3D5, (addr & 0xFF) as u8);
        super::ports::outb(0x3D4, 0x0E);
        super::ports::outb(0x3D5, ((addr >> 8) & 0xFF) as u8);
    }

    /// Gets the cursor's location.
    pub fn get_cursor_location(self) -> (u32, u32) {
        let mut addr: u32 = 0;

        super::ports::outb(0x3D4, 0x0F);
        addr |= super::ports::inb(0x3D5) as u32;

        super::ports::outb(0x3D4, 0x0E);
        addr |= (super::ports::inb(0x3D5) as u32) << 8;

        return (addr % self.width, addr / self.width);
    }
}
