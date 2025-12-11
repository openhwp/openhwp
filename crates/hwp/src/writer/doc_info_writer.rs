//! DocInfo 스트림 생성기
//!
//! DocInfo 스트림은 문서 수준의 정보를 포함하는 레코드들의 시퀀스입니다.
//!
//! # 레코드 순서
//!
//! 1. DOCUMENT_PROPERTIES (0x010)
//! 2. ID_MAPPINGS (0x011)
//! 3. BIN_DATA (0x012) × binary_data_count
//! 4. FACE_NAME (0x013) × total_font_count
//! 5. BORDER_FILL (0x014) × border_fill_count
//! 6. CHAR_SHAPE (0x015) × char_shape_count
//! 7. TAB_DEF (0x016) × tab_def_count
//! 8. NUMBERING (0x017) × numbering_count
//! 9. BULLET (0x018) × bullet_count
//! 10. PARA_SHAPE (0x019) × para_shape_count
//! 11. STYLE (0x01A) × style_count

use super::ByteWriter;
use crate::primitive::RecordTagId;

/// DocInfo 스트림 생성기
pub struct DocInfoWriter {
    /// 섹션 수
    section_count: u16,
    /// 시작 페이지 번호
    page_start_number: u16,
    /// 폰트 이름들 (언어별)
    fonts: FontCounts,
    /// BorderFill 데이터들
    border_fills: Vec<BorderFillData>,
    /// 글자 모양 데이터들
    char_shapes: Vec<CharShapeData>,
    /// 탭 정의 데이터들
    tab_definitions: Vec<TabDefinitionData>,
    /// 번호 매기기 데이터들
    numberings: Vec<NumberingData>,
    /// 글머리표 데이터들
    bullets: Vec<BulletData>,
    /// 문단 모양 데이터들
    para_shapes: Vec<ParaShapeData>,
    /// 스타일 데이터들
    styles: Vec<StyleData>,
    /// 바이너리 데이터 정보들
    binary_data: Vec<BinaryDataInfo>,
}

/// 언어별 폰트 수
#[derive(Debug, Clone, Default)]
pub struct FontCounts {
    /// 한글 폰트들
    pub korean: Vec<FontData>,
    /// 영문 폰트들
    pub english: Vec<FontData>,
    /// 한자 폰트들
    pub chinese: Vec<FontData>,
    /// 일본어 폰트들
    pub japanese: Vec<FontData>,
    /// 기타 폰트들
    pub other: Vec<FontData>,
    /// 기호 폰트들
    pub symbol: Vec<FontData>,
    /// 사용자 폰트들
    pub user: Vec<FontData>,
}

/// 폰트 데이터
#[derive(Debug, Clone)]
pub struct FontData {
    /// 폰트 이름
    pub name: String,
    /// 대체 폰트 타입
    pub alternate_type: Option<u8>,
    /// 대체 폰트 이름
    pub alternate_name: Option<String>,
    /// PANOSE 정보
    pub panose_info: Option<PanoseData>,
    /// 기본 폰트 이름
    pub default_font_name: Option<String>,
}

/// PANOSE 글꼴 분류 정보
#[derive(Debug, Clone, Copy)]
pub struct PanoseData {
    /// 폰트 패밀리 종류
    pub family_kind: u8,
    /// 세리프 스타일
    pub serif_style: u8,
    /// 굵기
    pub weight: u8,
    /// 비례
    pub proportion: u8,
    /// 대조
    pub contrast: u8,
    /// 스트로크 편차
    pub stroke_variation: u8,
    /// 자획 스타일
    pub arm_style: u8,
    /// 글자형
    pub letterform: u8,
    /// 중간선
    pub midline: u8,
    /// X-높이
    pub x_height: u8,
}

/// BorderFill 데이터
#[derive(Debug, Clone)]
pub struct BorderFillData {
    /// 속성
    pub properties: u16,
    /// 왼쪽 테두리
    pub left_border: BorderLine,
    /// 오른쪽 테두리
    pub right_border: BorderLine,
    /// 위쪽 테두리
    pub top_border: BorderLine,
    /// 아래쪽 테두리
    pub bottom_border: BorderLine,
    /// 대각선
    pub diagonal: BorderLine,
    /// 채우기 타입
    pub fill_type: u32,
    /// 배경색
    pub background_color: u32,
}

/// 테두리 선
#[derive(Debug, Clone, Default)]
pub struct BorderLine {
    /// 선 종류
    pub style: u8,
    /// 선 굵기
    pub thickness: u8,
    /// 선 색상
    pub color: u32,
}

/// 글자 모양 데이터
#[derive(Debug, Clone)]
pub struct CharShapeData {
    /// 폰트 ID들 (언어별 7개)
    pub font_ids: [u16; 7],
    /// 폰트 크기 비율들 (언어별 7개, 100 = 100%)
    pub font_ratios: [u8; 7],
    /// 폰트 간격들 (언어별 7개)
    pub font_spacings: [i8; 7],
    /// 상대 크기들 (언어별 7개, 100 = 100%)
    pub relative_sizes: [u8; 7],
    /// 문자 위치들 (언어별 7개, -100 ~ 100)
    pub positions: [i8; 7],
    /// 폰트 크기 (pt × 100)
    pub font_size: i32,
    /// 속성
    pub properties: u32,
    /// 그림자 간격 X (퍼센트, -100 ~ 100)
    pub shadow_offset_x: i8,
    /// 그림자 간격 Y (퍼센트, -100 ~ 100)
    pub shadow_offset_y: i8,
    /// 글자 색상
    pub text_color: u32,
    /// 밑줄 색상
    pub underline_color: u32,
    /// 음영 색상
    pub shade_color: u32,
    /// 그림자 색상
    pub shadow_color: u32,
    /// 테두리/배경 ID (Optional, version 5.0.2.1+)
    pub border_fill_id: Option<u16>,
    /// 취소선 색상 (Optional, version 5.0.3.0+)
    pub strikethrough_color: Option<u32>,
}

/// 탭 정의 데이터
#[derive(Debug, Clone, Default)]
pub struct TabDefinitionData {
    /// 속성
    pub properties: u32,
    /// 탭 항목들
    pub items: Vec<TabItem>,
}

/// 탭 항목
#[derive(Debug, Clone)]
pub struct TabItem {
    /// 위치
    pub position: u32,
    /// 종류
    pub tab_type: u8,
    /// 채움 문자
    pub fill_char: u16,
}

/// 번호 매기기 데이터
#[derive(Debug, Clone)]
pub struct NumberingData {
    /// 레벨별 정보 (최대 7레벨)
    pub levels: Vec<NumberingLevelData>,
    /// 시작 번호
    pub start_number: u16,
}

/// 번호 매기기 레벨 데이터
#[derive(Debug, Clone, Default)]
pub struct NumberingLevelData {
    /// 정렬 방식 (0=Left, 1=Center, 2=Right)
    pub alignment: u8,
    /// 실제 인스턴스 너비 사용 여부
    pub use_instance_width: bool,
    /// 자동 들여쓰기 여부
    pub auto_indent: bool,
    /// 번호 형식 타입 (0=Digit, 2=RomanUpper, 8=Hangul 등)
    pub number_format: u8,
    /// 번호 너비 보정값
    pub width_correction: i16,
    /// 텍스트 오프셋 (번호와 텍스트 사이 거리)
    pub text_distance: i16,
    /// 글자 모양 ID
    pub char_shape_id: u32,
    /// 번호 형식 문자열
    pub format: String,
    /// 레벨별 시작 번호
    pub start_number: u32,
}

/// 글머리표 데이터
#[derive(Debug, Clone)]
pub struct BulletData {
    /// 문단 머리 정보
    pub para_head_info: u32,
    /// 글머리 문자
    pub bullet_char: char,
}

/// 문단 모양 데이터
#[derive(Debug, Clone)]
pub struct ParaShapeData {
    /// 속성1
    pub properties1: u32,
    /// 왼쪽 여백
    pub margin_left: i32,
    /// 오른쪽 여백
    pub margin_right: i32,
    /// 들여쓰기
    pub indent: i32,
    /// 문단 위 간격
    pub space_before: i32,
    /// 문단 아래 간격
    pub space_after: i32,
    /// 줄 간격 (old)
    pub line_spacing: i16,
    /// 탭 정의 ID
    pub tab_def_id: u16,
    /// 번호 매기기/글머리표 ID
    pub numbering_bullet_id: u16,
    /// 테두리/채우기 ID
    pub border_fill_id: u16,
    /// 테두리 왼쪽 간격
    pub border_space_left: i16,
    /// 테두리 오른쪽 간격
    pub border_space_right: i16,
    /// 테두리 위쪽 간격
    pub border_space_top: i16,
    /// 테두리 아래쪽 간격
    pub border_space_bottom: i16,
    /// 속성2 (v5.0.1.7+)
    pub properties2: u32,
    /// 속성3 (v5.0.2.5+)
    pub properties3: u32,
    /// 줄 간격 (new, v5.0.2.5+)
    pub line_spacing2: u32,
}

/// 스타일 데이터
#[derive(Debug, Clone)]
pub struct StyleData {
    /// 스타일 이름
    pub name: String,
    /// 영문 이름
    pub english_name: String,
    /// 스타일 종류 (0=문단, 1=글자)
    pub style_type: u8,
    /// 다음 스타일 ID
    pub next_style_id: u8,
    /// 언어 코드
    pub lang_id: i16,
    /// 문단 모양 ID
    pub para_shape_id: u16,
    /// 글자 모양 ID
    pub char_shape_id: u16,
}

/// 바이너리 데이터 정보
#[derive(Debug, Clone)]
pub struct BinaryDataInfo {
    /// 타입 (링크, 임베드, 스토리지)
    pub data_type: u16,
    /// 절대 경로
    pub abs_path: String,
    /// 상대 경로
    pub rel_path: String,
    /// 바이너리 데이터 ID
    pub bin_data_id: u16,
    /// 확장자
    pub extension: String,
}

impl DocInfoWriter {
    /// 새 DocInfoWriter를 생성합니다.
    pub fn new(section_count: u16) -> Self {
        Self {
            section_count,
            page_start_number: 1,
            fonts: FontCounts::default(),
            border_fills: Vec::new(),
            char_shapes: Vec::new(),
            tab_definitions: Vec::new(),
            numberings: Vec::new(),
            bullets: Vec::new(),
            para_shapes: Vec::new(),
            styles: Vec::new(),
            binary_data: Vec::new(),
        }
    }

    /// 시작 페이지 번호를 설정합니다.
    pub fn with_page_start_number(mut self, num: u16) -> Self {
        self.page_start_number = num;
        self
    }

    /// 폰트를 설정합니다.
    pub fn set_fonts(&mut self, fonts: FontCounts) {
        self.fonts = fonts;
    }

    /// BorderFill을 추가합니다.
    pub fn add_border_fill(&mut self, data: BorderFillData) {
        self.border_fills.push(data);
    }

    /// 글자 모양을 추가합니다.
    pub fn add_char_shape(&mut self, data: CharShapeData) {
        self.char_shapes.push(data);
    }

    /// 탭 정의를 추가합니다.
    pub fn add_tab_definition(&mut self, data: TabDefinitionData) {
        self.tab_definitions.push(data);
    }

    /// 번호 매기기를 추가합니다.
    pub fn add_numbering(&mut self, data: NumberingData) {
        self.numberings.push(data);
    }

    /// 글머리표를 추가합니다.
    pub fn add_bullet(&mut self, data: BulletData) {
        self.bullets.push(data);
    }

    /// 문단 모양을 추가합니다.
    pub fn add_para_shape(&mut self, data: ParaShapeData) {
        self.para_shapes.push(data);
    }

    /// 스타일을 추가합니다.
    pub fn add_style(&mut self, data: StyleData) {
        self.styles.push(data);
    }

    /// 바이너리 데이터 정보를 추가합니다.
    pub fn add_binary_data_info(&mut self, info: BinaryDataInfo) {
        self.binary_data.push(info);
    }

    /// DocInfo 데이터를 빌드합니다.
    pub fn build(&self) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        // 1. DOCUMENT_PROPERTIES
        self.write_document_properties(&mut writer);

        // 2. ID_MAPPINGS
        self.write_id_mappings(&mut writer);

        // 3. BIN_DATA
        for bin_info in &self.binary_data {
            self.write_binary_data(&mut writer, bin_info);
        }

        // 4. FACE_NAME (언어별로)
        self.write_face_names(&mut writer);

        // 5. BORDER_FILL
        for bf in &self.border_fills {
            self.write_border_fill(&mut writer, bf);
        }

        // 6. CHAR_SHAPE
        for cs in &self.char_shapes {
            self.write_char_shape(&mut writer, cs);
        }

        // 7. TAB_DEF
        for td in &self.tab_definitions {
            self.write_tab_definition(&mut writer, td);
        }

        // 8. NUMBERING
        for num in &self.numberings {
            self.write_numbering(&mut writer, num);
        }

        // 9. BULLET
        for bullet in &self.bullets {
            self.write_bullet(&mut writer, bullet);
        }

        // 10. PARA_SHAPE
        for ps in &self.para_shapes {
            self.write_para_shape(&mut writer, ps);
        }

        // 11. STYLE
        for style in &self.styles {
            self.write_style(&mut writer, style);
        }

        writer.into_bytes()
    }

    fn write_document_properties(&self, writer: &mut ByteWriter) {
        let mut data = ByteWriter::new();

        data.write_u16(self.section_count);
        data.write_u16(self.page_start_number);
        data.write_u16(1); // footnote_start_number
        data.write_u16(1); // endnote_start_number
        data.write_u16(1); // figure_start_number
        data.write_u16(1); // table_start_number
        data.write_u16(1); // equation_start_number
        data.write_u32(0); // caret_list_id
        data.write_u32(0); // caret_paragraph_id
        data.write_u32(0); // caret_position_in_paragraph

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::DocumentProperties, 0, &data_bytes);
    }

    fn write_id_mappings(&self, writer: &mut ByteWriter) {
        let mut data = ByteWriter::new();

        // 기본 카운트들 (15개 i32)
        data.write_i32(self.binary_data.len() as i32);
        data.write_i32(self.fonts.korean.len() as i32);
        data.write_i32(self.fonts.english.len() as i32);
        data.write_i32(self.fonts.chinese.len() as i32);
        data.write_i32(self.fonts.japanese.len() as i32);
        data.write_i32(self.fonts.other.len() as i32);
        data.write_i32(self.fonts.symbol.len() as i32);
        data.write_i32(self.fonts.user.len() as i32);
        data.write_i32(self.border_fills.len() as i32);
        data.write_i32(self.char_shapes.len() as i32);
        data.write_i32(self.tab_definitions.len() as i32);
        data.write_i32(self.numberings.len() as i32);
        data.write_i32(self.bullets.len() as i32);
        data.write_i32(self.para_shapes.len() as i32);
        data.write_i32(self.styles.len() as i32);

        // 확장 카운트들 (v5.0.2.1+)
        data.write_i32(0); // memo_shape_count
        data.write_i32(0); // track_change_count
        data.write_i32(0); // track_change_author_count

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::IdMappings, 0, &data_bytes);
    }

    fn write_binary_data(&self, writer: &mut ByteWriter, info: &BinaryDataInfo) {
        let mut data = ByteWriter::new();

        data.write_u16(info.data_type);

        // 타입에 따라 다른 데이터 쓰기
        match info.data_type & 0x0F {
            0 => {
                // 링크
                data.write_hwp_string(&info.abs_path);
                data.write_hwp_string(&info.rel_path);
            }
            1 => {
                // 임베드
                data.write_u16(info.bin_data_id);
                data.write_hwp_string(&info.extension);
            }
            2 => {
                // 스토리지
                data.write_u16(info.bin_data_id);
                data.write_hwp_string(&info.extension);
            }
            _ => {}
        }

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::BinaryData, 0, &data_bytes);
    }

    fn write_face_names(&self, writer: &mut ByteWriter) {
        // 각 언어별 폰트 쓰기
        for font_data in &self.fonts.korean {
            self.write_face_name(writer, font_data);
        }
        for font_data in &self.fonts.english {
            self.write_face_name(writer, font_data);
        }
        for font_data in &self.fonts.chinese {
            self.write_face_name(writer, font_data);
        }
        for font_data in &self.fonts.japanese {
            self.write_face_name(writer, font_data);
        }
        for font_data in &self.fonts.other {
            self.write_face_name(writer, font_data);
        }
        for font_data in &self.fonts.symbol {
            self.write_face_name(writer, font_data);
        }
        for font_data in &self.fonts.user {
            self.write_face_name(writer, font_data);
        }
    }

    fn write_face_name(&self, writer: &mut ByteWriter, font: &FontData) {
        const HAS_ALTERNATE: u8 = 0x80;
        const HAS_TYPE_INFO: u8 = 0x40;
        const HAS_DEFAULT: u8 = 0x20;

        let mut data = ByteWriter::new();

        // 속성 플래그 계산
        let mut properties = 0u8;
        if font.alternate_name.is_some() {
            properties |= HAS_ALTERNATE;
        }
        if font.panose_info.is_some() {
            properties |= HAS_TYPE_INFO;
        }
        if font.default_font_name.is_some() {
            properties |= HAS_DEFAULT;
        }

        data.write_u8(properties);

        // 폰트 이름
        data.write_hwp_string(&font.name);

        // 대체 폰트 정보 (있으면)
        if let Some(ref alt_name) = font.alternate_name {
            data.write_u8(font.alternate_type.unwrap_or(1)); // 기본값: TrueType
            data.write_hwp_string(alt_name);
        }

        // PANOSE 정보 (있으면)
        if let Some(panose) = font.panose_info {
            data.write_u8(panose.family_kind);
            data.write_u8(panose.serif_style);
            data.write_u8(panose.weight);
            data.write_u8(panose.proportion);
            data.write_u8(panose.contrast);
            data.write_u8(panose.stroke_variation);
            data.write_u8(panose.arm_style);
            data.write_u8(panose.letterform);
            data.write_u8(panose.midline);
            data.write_u8(panose.x_height);
        }

        // 기본 폰트 이름 (있으면)
        if let Some(ref default_name) = font.default_font_name {
            data.write_hwp_string(default_name);
        }

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::FaceName, 0, &data_bytes);
    }

    fn write_border_fill(&self, writer: &mut ByteWriter, bf: &BorderFillData) {
        let mut data = ByteWriter::new();

        data.write_u16(bf.properties);

        // 테두리 선들
        self.write_border_line(&mut data, &bf.left_border);
        self.write_border_line(&mut data, &bf.right_border);
        self.write_border_line(&mut data, &bf.top_border);
        self.write_border_line(&mut data, &bf.bottom_border);
        self.write_border_line(&mut data, &bf.diagonal);

        // 채우기
        data.write_u32(bf.fill_type);
        if bf.fill_type != 0 {
            data.write_u32(bf.background_color);
        }

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::BorderFill, 0, &data_bytes);
    }

    fn write_border_line(&self, data: &mut ByteWriter, line: &BorderLine) {
        data.write_u8(line.style);
        data.write_u8(line.thickness);
        data.write_u32(line.color);
    }

    fn write_char_shape(&self, writer: &mut ByteWriter, cs: &CharShapeData) {
        let mut data = ByteWriter::new();

        // 폰트 ID들 (7개)
        for id in &cs.font_ids {
            data.write_u16(*id);
        }

        // 폰트 크기 비율들 (7개)
        for ratio in &cs.font_ratios {
            data.write_u8(*ratio);
        }

        // 폰트 간격들 (7개)
        for spacing in &cs.font_spacings {
            data.write_i8(*spacing);
        }

        // 상대 크기 (7개)
        for size in &cs.relative_sizes {
            data.write_u8(*size);
        }

        // 글자 위치 (7개)
        for pos in &cs.positions {
            data.write_i8(*pos);
        }

        // 기본 폰트 크기
        data.write_i32(cs.font_size);

        // 속성
        data.write_u32(cs.properties);

        // 그림자 간격 (INT8, 퍼센트 단위)
        data.write_i8(cs.shadow_offset_x);
        data.write_i8(cs.shadow_offset_y);

        // 색상들
        data.write_u32(cs.text_color);
        data.write_u32(cs.underline_color);
        data.write_u32(cs.shade_color);
        data.write_u32(cs.shadow_color);

        // 테두리/배경 ID (Optional, version 5.0.2.1+)
        if let Some(border_fill_id) = cs.border_fill_id {
            data.write_u16(border_fill_id);
        }

        // 취소선 색상 (Optional, version 5.0.3.0+)
        if let Some(strikethrough_color) = cs.strikethrough_color {
            data.write_u32(strikethrough_color);
        }

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::CharacterShape, 0, &data_bytes);
    }

    fn write_tab_definition(&self, writer: &mut ByteWriter, td: &TabDefinitionData) {
        let mut data = ByteWriter::new();

        data.write_u32(td.properties);
        data.write_u32(td.items.len() as u32);

        for item in &td.items {
            data.write_u32(item.position);
            data.write_u8(item.tab_type);
            data.write_u16(item.fill_char);
            data.write_u8(0); // reserved
        }

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::TabDefinition, 0, &data_bytes);
    }

    fn write_numbering(&self, writer: &mut ByteWriter, num: &NumberingData) {
        let mut data = ByteWriter::new();

        // 레벨 정보 (최대 7레벨)
        for level in &num.levels {
            // ParagraphHeadInfo 비트 필드 구성 (12 bytes)
            let mut properties: u32 = 0;
            properties |= (level.alignment as u32) & 0x03;           // 비트 [1:0] 정렬
            if level.use_instance_width {
                properties |= 1 << 2;                                 // 비트 [2] 인스턴스 너비 사용
            }
            if level.auto_indent {
                properties |= 1 << 3;                                 // 비트 [3] 자동 들여쓰기
            }
            properties |= ((level.number_format as u32) & 0x0F) << 12; // 비트 [15:12] 번호 형식

            data.write_u32(properties);
            data.write_i16(level.width_correction);
            data.write_i16(level.text_distance);
            data.write_u32(level.char_shape_id);
            data.write_hwp_string(&level.format);
        }

        // 패딩 (2 bytes)
        data.write_u16(num.start_number);

        // 레벨별 시작 번호 (7 x u32)
        for level in &num.levels {
            data.write_u32(level.start_number);
        }

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::Numbering, 0, &data_bytes);
    }

    fn write_bullet(&self, writer: &mut ByteWriter, bullet: &BulletData) {
        let mut data = ByteWriter::new();

        data.write_u32(bullet.para_head_info);
        data.write_u16(bullet.bullet_char as u16);

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::Bullet, 0, &data_bytes);
    }

    fn write_para_shape(&self, writer: &mut ByteWriter, ps: &ParaShapeData) {
        let mut data = ByteWriter::new();

        data.write_u32(ps.properties1);
        data.write_i32(ps.margin_left);
        data.write_i32(ps.margin_right);
        data.write_i32(ps.indent);
        data.write_i32(ps.space_before);
        data.write_i32(ps.space_after);
        data.write_i16(ps.line_spacing);
        data.write_u16(ps.tab_def_id);
        data.write_u16(ps.numbering_bullet_id);
        data.write_u16(ps.border_fill_id);
        data.write_i16(ps.border_space_left);
        data.write_i16(ps.border_space_right);
        data.write_i16(ps.border_space_top);
        data.write_i16(ps.border_space_bottom);

        // v5.0.1.7+
        data.write_u32(ps.properties2);

        // v5.0.2.5+
        data.write_u32(ps.properties3);
        data.write_u32(ps.line_spacing2);

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::ParagraphShape, 0, &data_bytes);
    }

    fn write_style(&self, writer: &mut ByteWriter, style: &StyleData) {
        let mut data = ByteWriter::new();

        data.write_hwp_string(&style.name);
        data.write_hwp_string(&style.english_name);
        data.write_u8(style.style_type);
        data.write_u8(style.next_style_id);
        data.write_i16(style.lang_id);
        data.write_u16(style.para_shape_id);
        data.write_u16(style.char_shape_id);

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::Style, 0, &data_bytes);
    }
}

impl Default for DocInfoWriter {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Default for BorderFillData {
    fn default() -> Self {
        Self {
            properties: 0,
            left_border: BorderLine::default(),
            right_border: BorderLine::default(),
            top_border: BorderLine::default(),
            bottom_border: BorderLine::default(),
            diagonal: BorderLine::default(),
            fill_type: 0,
            background_color: 0xFFFFFF,
        }
    }
}

impl Default for CharShapeData {
    fn default() -> Self {
        Self {
            font_ids: [0; 7],
            font_ratios: [100; 7],
            font_spacings: [0; 7],
            relative_sizes: [100; 7],
            positions: [0; 7],
            font_size: 1000, // 10pt
            properties: 0,
            shadow_offset_x: 0,
            shadow_offset_y: 0,
            text_color: 0x000000,
            underline_color: 0x000000,
            shade_color: 0xFFFFFF,
            shadow_color: 0x808080,
            border_fill_id: None,
            strikethrough_color: None,
        }
    }
}

impl Default for ParaShapeData {
    fn default() -> Self {
        Self {
            properties1: 0,
            margin_left: 0,
            margin_right: 0,
            indent: 0,
            space_before: 0,
            space_after: 0,
            line_spacing: 160, // 160%
            tab_def_id: 0,
            numbering_bullet_id: 0,
            border_fill_id: 0,
            border_space_left: 0,
            border_space_right: 0,
            border_space_top: 0,
            border_space_bottom: 0,
            properties2: 0,
            properties3: 0,
            line_spacing2: 0,
        }
    }
}

impl Default for StyleData {
    fn default() -> Self {
        Self {
            name: "바탕글".to_string(),
            english_name: "Normal".to_string(),
            style_type: 0, // 문단 스타일
            next_style_id: 0,
            lang_id: 0x0412, // Korean
            para_shape_id: 0,
            char_shape_id: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_doc_info() {
        let writer = DocInfoWriter::new(1);
        let data = writer.build();

        // DOCUMENT_PROPERTIES 레코드가 있어야 함
        assert!(!data.is_empty());

        // 첫 번째 레코드 헤더 확인
        let header = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let tag = header & 0x3FF;
        assert_eq!(tag, RecordTagId::DocumentProperties.as_u16() as u32);
    }

    #[test]
    fn test_doc_info_with_fonts() {
        let mut writer = DocInfoWriter::new(1);
        writer.set_fonts(FontCounts {
            korean: vec![FontData {
                name: "함초롬돋움".to_string(),
                alternate_type: None,
                alternate_name: None,
                panose_info: None,
                default_font_name: None,
            }],
            english: vec![FontData {
                name: "Arial".to_string(),
                alternate_type: None,
                alternate_name: None,
                panose_info: None,
                default_font_name: None,
            }],
            ..Default::default()
        });

        let data = writer.build();
        assert!(!data.is_empty());
    }
}
