use std::{fs, io::Write};

static MULTIBOOT_HEADER: [u16; 14] = [
    // Magic fields
    0xE852, 0x50D6, // Magic number
    0x0000, 0x0000, // Architecture, 0=i386
    0x0000, 0x000E, // length of MULTIBOOT_HEADER
    0x17AD, 0xAF1C, // checksum=all magic field excluding this+this=0
    
    // Framebuffer tag- empty flags, no preference for width, height, or bit depth
    0x0005, 0x0000,
    0x0014, 0x0000,
    0x0000, 0x0000
];

fn from_u16(from: &mut [u16]) -> &[u8] {
    if cfg!(target_endian = "little") {
        for byte in from.iter_mut() {
            *byte = byte.to_be();
        }
    }

    let len = from.len().checked_mul(2).unwrap();
    let ptr: *const u8 = from.as_ptr().cast();
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

fn main() {
    let path = "./kernel.flat";
    let mut buf = fs::read(path).unwrap();
    buf = [
        from_u16(&mut (MULTIBOOT_HEADER.clone())).to_vec(),
        buf
    ].concat();
    fs::OpenOptions::new()
        .write(true)
        .open(path)
        .unwrap()
        .write(buf.as_mut_slice())
        .unwrap();
}
