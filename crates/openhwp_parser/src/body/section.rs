use crate::{decompress, u32};
use crate::{BodyIter, Compressed, HwpDocumentError, HwpTag, Paragraph, Record, Version};

#[derive(Debug)]
pub struct Section {
    pub paragraphs: Vec<Paragraph>,
}

impl Section {
    #[inline]
    pub fn from_non_distributed(
        buf: Vec<u8>,
        compressed: Compressed,
        version: &Version,
    ) -> Result<Self, HwpDocumentError> {
        let buf = decompress!(buf, compressed);

        Ok(Self::from_buf(&buf, version)?)
    }

    pub fn from_distributed(
        buf: Vec<u8>,
        compressed: Compressed,
        version: &Version,
    ) -> Result<Self, HwpDocumentError> {
        let decoded = decode(buf)?;
        let buf = decompress!(decoded, compressed);

        Ok(Self::from_buf(&buf, version)?)
    }

    pub fn from_buf(buf: &[u8], version: &Version) -> Result<Self, HwpDocumentError> {
        let mut stream = BodyIter::new(buf, version);
        let paragraphs = stream.paragraphs(version)?;

        Ok(Self { paragraphs })
    }
}

pub(crate) fn decode(buf: Vec<u8>) -> Result<Vec<u8>, HwpDocumentError> {
    let mut iter = Record::iter(&buf);
    let record = iter.expect(HwpTag::HWPTAG_DISTRIBUTE_DOC_DATA)?;

    match <[u8; 256]>::try_from(record.payload) {
        Ok(payload) => {
            let seed = u32(&payload, 0);
            let pseudo = pseudo(seed);
            let key = hash_code(seed, pseudo, payload);

            Ok(decode_aes_128_ecb(key, iter.remaining())?)
        }
        Err(_) => Err(HwpDocumentError::InvalidTagId(
            Some(record.tag),
            HwpTag::HWPTAG_DISTRIBUTE_DOC_DATA,
        )),
    }
}

pub(crate) const fn pseudo(mut seed: u32) -> [u8; 256] {
    let mut pseudo = [0; 256];
    let mut index = 0;

    loop {
        seed = seed.wrapping_mul(0x343FD).wrapping_add(0x269EC3);
        let fill = (seed >> 16) as u8;
        seed = seed.wrapping_mul(0x343FD).wrapping_add(0x269EC3);
        let mut times = ((seed >> 16) as u8 & 0x0f) + 1;

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

pub(crate) const fn hash_code(seed: u32, pseudo: [u8; 256], payload: [u8; 256]) -> [u8; 16] {
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
