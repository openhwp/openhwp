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
    let buf: Vec<_> = buf
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();

    String::from_utf16_lossy(&buf).to_string()
}

pub fn decompress(source: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    use flate2::bufread::DeflateDecoder;
    use std::io::Read;

    let mut buf = vec![];
    DeflateDecoder::new(source).read_to_end(&mut buf)?;

    Ok(buf)
}

#[macro_export]
macro_rules! decompress {
    ($source:expr, $compressed:expr) => {{
        match $compressed {
            crate::Compressed::Yes => crate::decompress(&$source)?,
            crate::Compressed::No => $source,
        }
    }};
}
