//! 바이트 쓰기 유틸리티
//!
//! HWP 레코드 데이터를 바이트로 직렬화하기 위한 헬퍼 타입입니다.

use crate::primitive::RecordTagId;

/// 바이트 버퍼에 데이터를 쓰기 위한 헬퍼
pub struct ByteWriter {
    buffer: Vec<u8>,
}

impl ByteWriter {
    /// 새 ByteWriter를 생성합니다.
    pub const fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// 내부 버퍼를 반환합니다.
    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }

    /// u8을 씁니다.
    pub fn write_u8(&mut self, value: u8) {
        self.buffer.push(value);
    }

    /// i8을 씁니다.
    pub fn write_i8(&mut self, value: i8) {
        self.buffer.push(value as u8);
    }

    /// u16을 리틀 엔디안으로 씁니다.
    pub fn write_u16(&mut self, value: u16) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    /// i16을 리틀 엔디안으로 씁니다.
    pub fn write_i16(&mut self, value: i16) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    /// u32를 리틀 엔디안으로 씁니다.
    pub fn write_u32(&mut self, value: u32) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    /// i32를 리틀 엔디안으로 씁니다.
    pub fn write_i32(&mut self, value: i32) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    /// 바이트 슬라이스를 씁니다.
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    /// HWP 문자열(UTF-16LE, 길이 포함)을 씁니다.
    pub fn write_hwp_string(&mut self, s: &str) {
        // 문자열 길이 (문자 수, u16)
        let utf16: Vec<u16> = s.encode_utf16().collect();
        self.write_u16(utf16.len() as u16);

        // UTF-16LE 데이터
        for ch in utf16 {
            self.write_u16(ch);
        }
    }

    /// HWP 레코드 헤더를 씁니다.
    ///
    /// 헤더 형식: Tag(10bits) | Level(10bits) | Size(12bits)
    /// 크기가 0xFFF를 초과하면 확장 크기를 사용합니다.
    pub fn write_record_header(&mut self, tag: RecordTagId, level: u16, data_size: u32) {
        let tag_value = tag.as_u16() as u32;
        let level_value = (level as u32) & 0x3FF;

        if data_size > 0xFFE {
            // 확장 크기 사용
            let header = tag_value | (level_value << 10) | (0xFFF << 20);
            self.write_u32(header);
            self.write_u32(data_size);
        } else {
            let header = tag_value | (level_value << 10) | (data_size << 20);
            self.write_u32(header);
        }
    }

    /// 완전한 레코드(헤더 + 데이터)를 씁니다.
    pub fn write_record(&mut self, tag: RecordTagId, level: u16, data: &[u8]) {
        self.write_record_header(tag, level, data.len() as u32);
        self.write_bytes(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_u8() {
        let mut writer = ByteWriter::new();
        writer.write_u8(0xAB);
        assert_eq!(writer.into_bytes(), vec![0xAB]);
    }

    #[test]
    fn test_write_u16() {
        let mut writer = ByteWriter::new();
        writer.write_u16(0x1234);
        assert_eq!(writer.into_bytes(), vec![0x34, 0x12]);
    }

    #[test]
    fn test_write_u32() {
        let mut writer = ByteWriter::new();
        writer.write_u32(0x12345678);
        assert_eq!(writer.into_bytes(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn test_write_hwp_string() {
        let mut writer = ByteWriter::new();
        writer.write_hwp_string("AB");
        let bytes = writer.into_bytes();
        // 길이(2) + 'A' + 'B' = 2 + 4 = 6 바이트
        assert_eq!(bytes.len(), 6);
        assert_eq!(bytes[0..2], [0x02, 0x00]); // 길이 = 2
        assert_eq!(bytes[2..4], [0x41, 0x00]); // 'A'
        assert_eq!(bytes[4..6], [0x42, 0x00]); // 'B'
    }

    #[test]
    fn test_write_record_header_small() {
        let mut writer = ByteWriter::new();
        // Tag=0x010, Level=0, Size=30
        writer.write_record_header(RecordTagId::DocumentProperties, 0, 30);
        let bytes = writer.into_bytes();
        assert_eq!(bytes.len(), 4);
        // 0x010 | (0 << 10) | (30 << 20) = 0x01E00010
        let header = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        assert_eq!(header & 0x3FF, 0x010); // tag
        assert_eq!((header >> 10) & 0x3FF, 0); // level
        assert_eq!((header >> 20) & 0xFFF, 30); // size
    }

    #[test]
    fn test_write_record_header_extended() {
        let mut writer = ByteWriter::new();
        // 확장 크기 테스트 (> 0xFFE)
        writer.write_record_header(RecordTagId::DocumentProperties, 0, 10000);
        let bytes = writer.into_bytes();
        assert_eq!(bytes.len(), 8);
        // 헤더: size 필드가 0xFFF
        let header = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        assert_eq!((header >> 20) & 0xFFF, 0xFFF);
        // 확장 크기
        let ext_size = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        assert_eq!(ext_size, 10000);
    }
}
