pub const fn u16(buf: &[u8], start_index: usize) -> u16 {
    <u16>::from_le_bytes([buf[start_index], buf[start_index + 1]])
}

pub const fn u32(buf: &[u8], start_index: usize) -> u32 {
    <u32>::from_le_bytes([
        buf[start_index],
        buf[start_index + 1],
        buf[start_index + 2],
        buf[start_index + 3],
    ])
}

pub fn to_string(buf: &[u8]) -> String {
    let buf = unsafe { std::mem::transmute(buf) };

    String::from_utf16_lossy(buf).to_string()
}
