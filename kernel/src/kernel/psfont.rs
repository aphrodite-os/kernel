//! PC Screen Font stuff

use alloc::vec::Vec;

/// The font selected to be the "main" font. I selected Linux's
/// ISO01-12x22 font.
pub const FONT1: &[u8; 12107] = include_bytes!("iso01-12x22.psfu");

/// PC Screen Font magic number.
pub const PSF_MAGIC: u32 = 0x864ab572;

/// A PC Screen font.
pub struct RawPCScreenFont {
    /// The magic number. Should be 0x864ab572.
    pub magic: u32,
    /// The version. Should be 0.
    pub version: u32,
    /// The size of the header/offset of bitmaps.
    pub header_size: u32,
    /// Flags. 0 if there's no unicode table.
    pub flags: u32,
    /// The number of glyphs.
    pub num_glyphs: u32,
    /// The number of bytes per glyph.
    pub bytes_per_glyph: u32,
    /// The height of each glyph.
    pub height: u32,
    /// The width of each glyph.
    pub width: u32,
    /// The glyphs.
    pub glyphs: Vec<u8>,
}

/// The glyph type for [PCScreenFont].
#[derive(Clone, Copy)]
pub struct Glyph {
    /// The size of this glyph.
    pub len: u32,
    /// The height of this glyph.
    pub height: u32,
    /// The width of this glyph.
    pub width: u32,
    /// The raw glyph data(and unicode translation table).
    pub data: &'static [u8],
}

/// A more useful form of [RawPCScreenFont].
pub struct PCScreenFont {
    /// The version. Should be 0.
    pub version: u32,
    /// Flags. 0 if there's no unicode table.
    pub flags: u32,
    /// The height of each glyph.
    pub height: u32,
    /// The width of each glyph.
    pub width: u32,
    /// The glyphs.
    pub glyphs: Vec<Glyph>,
    /// The unicode translation table.
    pub unitable: Option<Vec<Vec<u8>>>,
}

/// Error code returned when the PSF has an invalid magic number.
pub const ERR_INVALID_MAGIC: i16 = -1;

/// Parses a PC Screen Font into a [RawPCScreenFont].
pub fn parse_raw_pc_screen_font(data: Vec<u8>) -> Result<RawPCScreenFont, crate::Error<'static>> {
    let out = RawPCScreenFont {
        magic: u32::from_le_bytes(data[0..3].try_into().unwrap()),
        version: u32::from_le_bytes(data[4..7].try_into().unwrap()),
        header_size: u32::from_le_bytes(data[8..11].try_into().unwrap()),
        flags: u32::from_le_bytes(data[12..15].try_into().unwrap()),
        num_glyphs: u32::from_le_bytes(data[16..20].try_into().unwrap()),
        bytes_per_glyph: u32::from_le_bytes(data[20..23].try_into().unwrap()),
        height: u32::from_le_bytes(data[24..27].try_into().unwrap()),
        width: u32::from_le_bytes(data[28..31].try_into().unwrap()),
        glyphs: data[32..].to_vec(),
    };
    if out.magic != PSF_MAGIC {
        return Err(crate::Error::new("Invalid magic", ERR_INVALID_MAGIC));
    }
    Ok(out)
}

/// Parses a PC Screen Font into a [PCScreenFont].
pub fn parse_pc_screen_font(data: RawPCScreenFont) -> Result<PCScreenFont, crate::Error<'static>> {
    unsafe {
        if data.flags == 0 {
            let mut unitable: Vec<Vec<u8>> =
                Vec::with_capacity(data.num_glyphs as usize * core::mem::size_of::<Vec<u8>>());

            let unistr = (data.glyphs.as_slice() as *const [u8])
                .byte_add(data.bytes_per_glyph as usize * data.num_glyphs as usize);

            let mut i = 0usize;
            let mut f = 0usize;
            loop {
                let g = i + f;
                if i >= data.num_glyphs as usize {
                    break;
                }
                let char = (*unistr)[g];
                if char == 0xFF {
                    i += 1;
                    f = 0;
                    continue;
                }
                unitable[i].push(char);
                f += 1;
            }

            let out = PCScreenFont {
                version: data.version,
                flags: data.flags,
                height: data.height,
                width: data.width,
                glyphs: Vec::from(core::mem::transmute::<&[u8], &[Glyph]>(
                    data.glyphs.as_slice(),
                )),
                unitable: Some(unitable),
            };
            return Ok(out);
        }

        let out = PCScreenFont {
            version: data.version,
            flags: data.flags,
            height: data.height,
            width: data.width,
            glyphs: Vec::from(core::mem::transmute::<&[u8], &[Glyph]>(
                data.glyphs.as_slice(),
            )),
            unitable: None,
        };
        Ok(out)
    }
}

/// Parses a Vec<u8> into a [PCScreenFont].
pub fn parse_psfu(data: Vec<u8>) -> Result<PCScreenFont, crate::Error<'static>> {
    let data = parse_raw_pc_screen_font(data)?;
    parse_pc_screen_font(data)
}
