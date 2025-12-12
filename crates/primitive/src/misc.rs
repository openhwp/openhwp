//! 기타 열거형

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 캡션 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CaptionPosition {
    /// 왼쪽
    Left,
    /// 오른쪽
    Right,
    /// 위
    Top,
    /// 아래
    #[default]
    Bottom,
}

/// 자동 번호 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AutoNumberType {
    /// 페이지 번호
    #[default]
    Page,
    /// 각주 번호
    Footnote,
    /// 미주 번호
    Endnote,
    /// 그림 번호
    Picture,
    /// 표 번호
    Table,
    /// 수식 번호
    Equation,
    /// 총 페이지
    TotalPages,
}

/// 개체 번호 매기기 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ObjectNumberingType {
    /// 없음
    #[default]
    None,
    /// 그림
    Picture,
    /// 표
    Table,
    /// 수식
    Equation,
}

/// 폰트 스타일
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FontStyle {
    /// 보통
    #[default]
    Regular,
    /// 굵게
    Bold,
    /// 기울임
    Italic,
    /// 굵은 기울임
    BoldItalic,
}

/// 개체 그림자 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ShapeShadowType {
    /// 없음
    #[default]
    None,
    /// 바깥 그림자
    Drop,
    /// 안쪽 그림자
    Inner,
}

/// 곡선 세그먼트 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CurveSegmentType {
    /// 직선
    #[default]
    Line,
    /// 곡선
    Curve,
}

/// 변경 추적 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrackChangeType {
    /// 알 수 없음
    #[default]
    Unknown,
    /// 텍스트 삽입
    Insert,
    /// 텍스트 삭제
    Delete,
    /// 서식 변경
    Format,
    /// 글자 모양 변경 (HWPX 전용)
    CharacterShape,
    /// 문단 모양 변경 (HWPX 전용)
    ParagraphShape,
}

/// 매개변수 종류 (문서 데이터)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParameterType {
    /// 알 수 없음
    #[default]
    Unknown,
    /// Null/빈 값
    Null,
    /// 불린 값
    Bool,
    /// 8비트 정수
    I1,
    /// 16비트 정수
    I2,
    /// 32비트 정수
    I4,
    /// 8비트 부호 없는 정수
    UI1,
    /// 16비트 부호 없는 정수
    UI2,
    /// 32비트 부호 없는 정수
    UI4,
    /// 유니코드 문자열
    String,
    /// 바이너리 데이터 참조
    BinaryData,
    /// 중첩 매개변수 집합
    Set,
    /// 매개변수 집합 배열
    Array,
}

/// 텍스트 오프셋 종류 (번호와 본문 거리 단위)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextOffsetType {
    /// 퍼센트
    #[default]
    Percent,
    /// HWP 유닛
    HwpUnit,
}
