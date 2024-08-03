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
