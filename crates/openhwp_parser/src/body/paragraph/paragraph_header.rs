use crate::{u16, u32, BodyIter, HwpDocumentError, HwpTag, Version};

#[derive(Debug)]
pub struct ParagraphHeader {
    /// text(=chars)
    pub text_size: usize,
    /// control mask<br>
    /// (UINT32)(1<<ctrlch) 조합<br>
    /// ctrlch는 HwpCtrlAPI.Hwp 2.1 CtrlCh 참고
    pub control_mask: ControlMask,
    pub paragraph_shape_id: u16,
    pub paragraph_style_id: u8,
    /// 구역 나누기
    pub section_break: bool,
    /// 다단 나누기
    pub columns_break: bool,
    /// 쪽 나누기
    pub page_break: bool,
    /// 단 나누기
    pub column_break: bool,
    /// 글자 모양 정보 수
    pub char_shape_count: u16,
    /// range tag 정보 수
    pub range_tag_count: u16,
    /// 각 줄에 대한 align에 대한 정보 수
    pub line_align_count: u16,
    /// 문단 Instance ID (unique ID)
    pub instance_id: u32,
    /// 변경추적 병합 문단여부. (5.0.3.2 버전 이상)
    pub tracking_change_merged: Option<bool>,
}

#[derive(Debug)]
pub struct ControlMask {
    // 1: Reserved
    /// 2: 구역/단 정의
    pub section_column_definition: bool,
    /// 3: 필드시작
    pub field_start: bool,
    /// 4: 필드끝
    pub field_end: bool,
    // 5: Reserved
    // 6: Reserved
    // 7: Reserved
    // 8: Reserved
    /// 9: 탭
    pub tab: bool,
    /// 10: 강제 줄 나눔
    pub line_break: bool,
    /// 11: 그리기 개체 / 표
    pub shape_object_table: bool,
    // 12: Reserved
    /// 13: 문단 나누기
    pub paragraph_break: bool,
    // 14: Reserved
    /// 15: 주석
    pub hidden_comment: bool,
    /// 16: 머리말 / 꼬리말 존재 여부
    pub header_footer: bool,
    /// 17: 각주 / 미주
    pub headnote_footnote: bool,
    /// 18: 자동 번호
    pub auto_number: bool,
    // 19: Reserved
    // 20: Reserved
    /// 21: 쪽바뀜
    pub page_break: bool,
    /// 22: 책갈피 / 찾아보기 표시
    pub book_mark_index_mark: bool,
    /// 23: 덧말 / 글자 겹침
    pub sub_text: bool,
    /// 24: 하이픈
    pub hyphen: bool,
    // 25: Reserved
    // 26: Reserved
    // 27: Reserved
    // 28: Reserved
    // 29: Reserved
    /// 30: 묶음 빈칸
    pub keep_word_space: bool,
    /// 31: 고정 폭 빈칸
    pub fixed_width_space: bool,
    // 32: Reserved
}

impl<'hwp> BodyIter<'hwp> {
    pub fn paragraph_header(
        &mut self,
        version: &Version,
    ) -> Result<ParagraphHeader, HwpDocumentError> {
        let record = self.expect(HwpTag::HWPTAG_PARA_HEADER)?;

        Ok(ParagraphHeader::from_buf(record.payload, version))
    }
}

impl ParagraphHeader {
    pub fn from_buf(buf: &[u8], version: &Version) -> Self {
        let (text_size, buf) = buf.split_at(4);
        let text_size = match u32(text_size, 0) {
            size if size & 0x80000000 == 0x80000000 => size & 0x7fffffff,
            size => size,
        } as usize;
        let (control_mask, buf) = buf.split_at(4);
        let control_mask = ControlMask::from_buf(control_mask);
        let paragraph_shape_id = u16(buf, 0);
        let paragraph_style_id = buf[2];
        let section_break = buf[3] & 0b0000_0001 != 0;
        let columns_break = buf[3] & 0b0000_0010 != 0;
        let page_break = buf[3] & 0b0000_0100 != 0;
        let column_break = buf[3] & 0b0000_1000 != 0;
        let char_shape_count = u16(buf, 4);
        let range_tag_count = u16(buf, 6);
        let line_align_count = u16(buf, 8);
        let instance_id = u32(buf, 10);
        let tracking_change_merged = if version >= &Version::V5_0_3_2 {
            Some(buf[14] != 0)
        } else {
            None
        };

        Self {
            text_size,
            control_mask,
            paragraph_shape_id,
            paragraph_style_id,
            section_break,
            columns_break,
            page_break,
            column_break,
            char_shape_count,
            range_tag_count,
            line_align_count,
            instance_id,
            tracking_change_merged,
        }
    }
}

impl ControlMask {
    pub const fn from_buf(buf: &[u8]) -> Self {
        Self {
            section_column_definition: buf[0] & 0b0000_0010 != 0,
            field_start: buf[0] & 0b0000_0100 != 0,
            field_end: buf[0] & 0b0000_1000 != 0,
            tab: buf[1] & 0b0000_0001 != 0,
            line_break: buf[1] & 0b0000_0010 != 0,
            shape_object_table: buf[1] & 0b0000_0100 != 0,
            paragraph_break: buf[1] & 0b0001_0000 != 0,
            hidden_comment: buf[1] & 0b0100_0000 != 0,
            header_footer: buf[1] & 0b1000_0000 != 0,
            headnote_footnote: buf[2] & 0b0000_0001 != 0,
            auto_number: buf[2] & 0b0000_0010 != 0,
            page_break: buf[2] & 0b0001_0000 != 0,
            book_mark_index_mark: buf[2] & 0b0010_0000 != 0,
            sub_text: buf[2] & 0b0100_0000 != 0,
            hyphen: buf[2] & 0b1000_0000 != 0,
            keep_word_space: buf[3] & 0b0100_0000 != 0,
            fixed_width_space: buf[3] & 0b1000_0000 != 0,
        }
    }
}
