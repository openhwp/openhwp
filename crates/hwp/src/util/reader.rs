//! Byte reader utility for parsing binary data.
//!
//! Provides convenient methods for reading little-endian values
//! and UTF-16LE strings from byte slices.

use crate::error::{Error, Result};
use crate::primitive::{ColorReference, HwpUnit16};
use primitive::HwpUnit;

/// A reader for parsing binary data from a byte slice.
///
/// All multi-byte values are read in little-endian format.
#[derive(Debug, Clone)]
pub struct ByteReader<'a> {
    /// The underlying byte slice.
    data: &'a [u8],
    /// Current read position.
    position: usize,
}

impl<'a> ByteReader<'a> {
    /// Creates a new ByteReader from a byte slice.
    #[inline]
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    /// Returns the underlying byte slice.
    #[inline]
    pub const fn data(&self) -> &'a [u8] {
        self.data
    }

    /// Returns the current read position.
    #[inline]
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Returns the number of remaining bytes.
    #[inline]
    pub const fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.position)
    }

    /// Returns true if there are no more bytes to read.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.position >= self.data.len()
    }

    /// Seeks to an absolute position.
    #[inline]
    pub fn seek(&mut self, position: usize) -> Result<()> {
        if position > self.data.len() {
            return Err(Error::UnexpectedEndOfData {
                expected: position,
                actual: self.data.len(),
            });
        }
        self.position = position;
        Ok(())
    }

    /// Skips a number of bytes.
    #[inline]
    pub fn skip(&mut self, count: usize) -> Result<()> {
        self.ensure_available(count)?;
        self.position += count;
        Ok(())
    }

    /// Ensures at least `count` bytes are available.
    #[inline]
    fn ensure_available(&self, count: usize) -> Result<()> {
        if self.remaining() < count {
            return Err(Error::UnexpectedEndOfData {
                expected: count,
                actual: self.remaining(),
            });
        }
        Ok(())
    }

    /// Reads a single byte.
    #[inline]
    pub fn read_u8(&mut self) -> Result<u8> {
        self.ensure_available(1)?;
        let value = self.data[self.position];
        self.position += 1;
        Ok(value)
    }

    /// Reads a signed byte.
    #[inline]
    pub fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    /// Reads a little-endian u16.
    #[inline]
    pub fn read_u16(&mut self) -> Result<u16> {
        self.ensure_available(2)?;
        let bytes = [self.data[self.position], self.data[self.position + 1]];
        self.position += 2;
        Ok(u16::from_le_bytes(bytes))
    }

    /// Reads a little-endian i16.
    #[inline]
    pub fn read_i16(&mut self) -> Result<i16> {
        self.ensure_available(2)?;
        let bytes = [self.data[self.position], self.data[self.position + 1]];
        self.position += 2;
        Ok(i16::from_le_bytes(bytes))
    }

    /// Reads a little-endian u32.
    #[inline]
    pub fn read_u32(&mut self) -> Result<u32> {
        self.ensure_available(4)?;
        let bytes = [
            self.data[self.position],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
        ];
        self.position += 4;
        Ok(u32::from_le_bytes(bytes))
    }

    /// Reads a little-endian i32.
    #[inline]
    pub fn read_i32(&mut self) -> Result<i32> {
        self.ensure_available(4)?;
        let bytes = [
            self.data[self.position],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
        ];
        self.position += 4;
        Ok(i32::from_le_bytes(bytes))
    }

    /// Reads a little-endian u64.
    #[inline]
    pub fn read_u64(&mut self) -> Result<u64> {
        self.ensure_available(8)?;
        let bytes = [
            self.data[self.position],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
            self.data[self.position + 4],
            self.data[self.position + 5],
            self.data[self.position + 6],
            self.data[self.position + 7],
        ];
        self.position += 8;
        Ok(u64::from_le_bytes(bytes))
    }

    /// Reads a little-endian f64.
    #[inline]
    pub fn read_f64(&mut self) -> Result<f64> {
        Ok(f64::from_bits(self.read_u64()?))
    }

    /// Reads an HwpUnit (i32 from u32 bytes).
    ///
    /// HWP 파일에서 단위 값은 unsigned로 저장되지만,
    /// 실제 사용 시 signed 연산이 필요하므로 i32로 변환합니다.
    #[inline]
    pub fn read_hwp_unit(&mut self) -> Result<HwpUnit> {
        Ok(HwpUnit::new(self.read_u32()? as i32))
    }

    /// Reads an HwpUnit (i32 from i32 bytes).
    ///
    /// 음수가 저장될 수 있는 위치/오프셋 값을 읽습니다.
    #[inline]
    pub fn read_signed_hwp_unit(&mut self) -> Result<HwpUnit> {
        Ok(HwpUnit::new(self.read_i32()?))
    }

    /// Reads an HwpUnit16 (i16).
    #[inline]
    pub fn read_hwp_unit16(&mut self) -> Result<HwpUnit16> {
        Ok(HwpUnit16::new(self.read_i16()?))
    }

    /// Reads a ColorReference (u32).
    #[inline]
    pub fn read_color(&mut self) -> Result<ColorReference> {
        Ok(ColorReference::new(self.read_u32()?))
    }

    /// Reads a fixed-size byte array.
    #[inline]
    pub fn read_bytes(&mut self, count: usize) -> Result<&'a [u8]> {
        self.ensure_available(count)?;
        let bytes = &self.data[self.position..self.position + count];
        self.position += count;
        Ok(bytes)
    }

    /// Reads remaining bytes.
    #[inline]
    pub fn read_remaining(&mut self) -> &'a [u8] {
        let bytes = &self.data[self.position..];
        self.position = self.data.len();
        bytes
    }

    /// Reads a UTF-16LE string with a length prefix (WORD).
    ///
    /// The length prefix indicates the number of WCHAR (2-byte) characters.
    pub fn read_utf16_string(&mut self) -> Result<String> {
        let char_count = self.read_u16()? as usize;
        self.read_utf16_string_fixed(char_count)
    }

    /// Reads a UTF-16LE string with a fixed character count.
    pub fn read_utf16_string_fixed(&mut self, char_count: usize) -> Result<String> {
        let byte_count = char_count * 2;
        self.ensure_available(byte_count)?;

        let mut chars = Vec::with_capacity(char_count);
        for _ in 0..char_count {
            let code_unit = self.read_u16()?;
            chars.push(code_unit);
        }

        String::from_utf16(&chars).map_err(|_| Error::InvalidUtf16String)
    }

    /// Reads a null-terminated UTF-16LE string.
    pub fn read_utf16_string_null_terminated(&mut self) -> Result<String> {
        let mut chars = Vec::new();
        loop {
            let code_unit = self.read_u16()?;
            if code_unit == 0 {
                break;
            }
            chars.push(code_unit);
        }

        String::from_utf16(&chars).map_err(|_| Error::InvalidUtf16String)
    }

    /// Reads a fixed-size array of u8.
    #[inline]
    pub fn read_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        self.ensure_available(N)?;
        let mut array = [0u8; N];
        array.copy_from_slice(&self.data[self.position..self.position + N]);
        self.position += N;
        Ok(array)
    }

    /// Creates a sub-reader for a portion of the data.
    #[inline]
    pub fn sub_reader(&mut self, length: usize) -> Result<ByteReader<'a>> {
        self.ensure_available(length)?;
        let sub_data = &self.data[self.position..self.position + length];
        self.position += length;
        Ok(ByteReader::new(sub_data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_integers() {
        let data = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
        let mut reader = ByteReader::new(&data);

        assert_eq!(reader.read_u8().unwrap(), 0x12);
        assert_eq!(reader.read_u16().unwrap(), 0x5634);
        assert_eq!(reader.position(), 3);

        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u32().unwrap(), 0x78563412);
    }

    #[test]
    fn test_read_utf16_string() {
        // "Hello" in UTF-16LE with length prefix
        let data = [
            0x05, 0x00, // length = 5
            0x48, 0x00, // 'H'
            0x65, 0x00, // 'e'
            0x6C, 0x00, // 'l'
            0x6C, 0x00, // 'l'
            0x6F, 0x00, // 'o'
        ];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_utf16_string().unwrap(), "Hello");
    }

    #[test]
    fn test_remaining() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.remaining(), 4);
        reader.read_u16().unwrap();
        assert_eq!(reader.remaining(), 2);
    }

    #[test]
    fn test_unexpected_end() {
        let data = [0x01];
        let mut reader = ByteReader::new(&data);
        assert!(reader.read_u32().is_err());
    }

    #[test]
    fn test_skip() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = ByteReader::new(&data);
        reader.skip(2).unwrap();
        assert_eq!(reader.read_u8().unwrap(), 0x03);
    }

    #[test]
    fn test_sub_reader() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05];
        let mut reader = ByteReader::new(&data);
        reader.skip(1).unwrap();

        let mut sub = reader.sub_reader(2).unwrap();
        assert_eq!(sub.read_u8().unwrap(), 0x02);
        assert_eq!(sub.read_u8().unwrap(), 0x03);
        assert!(sub.is_empty());

        assert_eq!(reader.read_u8().unwrap(), 0x04);
    }
}
