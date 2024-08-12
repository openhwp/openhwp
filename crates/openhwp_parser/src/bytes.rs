use crate::{DocInfoTag, HwpDocumentError, Record};

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

pub fn decode(buf: Vec<u8>) -> Result<Vec<u8>, HwpDocumentError> {
    let mut iter = Record::iter(&buf);
    let record = iter.expect(DocInfoTag::HWPTAG_DISTRIBUTE_DOC_DATA as u16)?;

    match <[u8; 256]>::try_from(record.payload) {
        Ok(payload) => {
            let seed = u32(&payload, 0);
            let pseudo = pseudo(seed);
            let key = hash_code(seed, pseudo, payload);

            Ok(decode_aes_128_ecb(key, iter.remaining())?)
        }
        Err(_) => Err(HwpDocumentError::InvalidTagId(
            DocInfoTag::HWPTAG_DISTRIBUTE_DOC_DATA as u16,
        )),
    }
}

const fn pseudo(mut seed: u32) -> [u8; 256] {
    let mut pseudo = [0; 256];
    let mut index = 0;
    loop {
        seed = seed.wrapping_mul(0x343FD).wrapping_add(0x269EC3);
        let fill = (seed >> 16) as u8;
        seed = seed.wrapping_mul(0x343FD).wrapping_add(0x269EC3);
        let mut times = (seed >> 16) as u8 & 0x0f + 1;
        loop {
            pseudo[index] = fill;
            match index {
                255 => return pseudo,
                _ => index += 1,
            }
            match times {
                1 => break,
                _ => times -= 1,
            }
        }
    }
}

const fn hash_code(seed: u32, pseudo: [u8; 256], payload: [u8; 256]) -> [u8; 16] {
    let offset = ((seed & 0x0f) + 4) as usize;
    let mut hash_code = [0; 16];
    let mut index = 0;

    loop {
        hash_code[index] = payload[offset + index] ^ pseudo[offset + index];
        match index {
            15 => return hash_code,
            _ => index += 1,
        }
    }
}

fn decode_aes_128_ecb(key: [u8; 16], buf: &[u8]) -> Result<Vec<u8>, HwpDocumentError> {
    use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
    use aes::Aes128;

    let mut blocks: Vec<_> = buf
        .chunks_exact(16)
        .map(GenericArray::clone_from_slice)
        .collect();
    let cipher = Aes128::new_from_slice(&key).unwrap();
    cipher.decrypt_blocks(&mut blocks);

    Ok(blocks.into_iter().flatten().map(|x| x as u8).collect())
}
