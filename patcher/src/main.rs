use std::{fs, io::Write};

static MULTIBOOT_HEADER: [u32; 6] = [
    // Magic fields
    0xE85250D6, // Magic number
    0x00000000, // Architecture, 0=i386
    0x00000018, // length of MULTIBOOT_HEADER
    0x17ADAF12, // checksum=all magic field excluding this+this=0

    // Ending tag- empty flags, size 8
    0x00000000,
    0x00000008
];

fn from_u32(from: &mut [u32]) -> &[u8] {
    if cfg!(target_endian = "big") {
        for byte in from.iter_mut() {
            *byte = byte.to_le();
        }
    }

    let len = from.len().checked_mul(4).unwrap();
    let ptr: *const u8 = from.as_ptr().cast();
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

fn main() {
    let path = "./kernel.flat";
    let mut buf = fs::read(path).unwrap();
    buf = [
        from_u32(&mut (MULTIBOOT_HEADER.clone())).to_vec(),
        vec![ // jump past patch text
            0xE9, 0x55, 0x00, 0x00, 0x00
        ],
        b"Multiboot2 header patched by Aphrodite ".to_vec(),
        b"APHROKERN: OSS at github.com/AverseABFun/Aphrodite".to_vec(),
        buf
    ].concat();
    fs::OpenOptions::new()
        .write(true)
        .open(path)
        .unwrap()
        .write(buf.as_mut_slice())
        .unwrap();
}
