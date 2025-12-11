//! [AI 생성] 런 요소 (문단 내 인라인 컨텐츠 집합)
//!
//! 단일 스타일 컨텍스트 안에서 텍스트·표·도형·컨트롤 등 다양한 개체를 순서대로 담는 블록입니다. `$value` 순서가 레이아웃에 직결되므로 변경 시 주의가 필요합니다. KS X 6101:2024 `paralist.xsd`.

use serde::{Deserialize, Serialize};

use super::control::Control;
use super::drawing::{Arc, ConnectLine, Curve, Ellipse, Line, Polygon, Rectangle, UnknownObject};
use super::form_control::{Button, ComboBox, Edit, ListBox, ScrollBar};
use super::ole_equation::{Container, Equation, Ole};
use super::picture::Picture;
use super::section_definition::SectionDefinition;
use super::table::Table;
use super::text::TextElement;
use super::text_art::{Compose, Dutmal, TextArt};
use super::video_chart::{Chart, Video};
use crate::core::types::CharShapeIdRef;

/// [AI 생성] 런 내용 항목
///
/// 원본: `run` 요소 내 choice 요소들
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RunContent {
    /// 구역 정의
    #[serde(rename = "secPr")]
    SectionDefinition(Box<SectionDefinition>),
    /// 컨트롤
    #[serde(rename = "ctrl")]
    Control(Control),
    /// 텍스트
    #[serde(rename = "t")]
    Text(TextElement),
    /// 표
    #[serde(rename = "tbl")]
    Table(Box<Table>),
    /// 그림
    #[serde(rename = "pic")]
    Picture(Box<Picture>),
    /// OLE 개체
    #[serde(rename = "ole")]
    Ole(Box<Ole>),
    /// 컨테이너
    #[serde(rename = "container")]
    Container(Box<Container>),
    /// 수식
    #[serde(rename = "equation")]
    Equation(Box<Equation>),
    /// 선
    #[serde(rename = "line")]
    Line(Box<Line>),
    /// 사각형
    #[serde(rename = "rect")]
    Rectangle(Box<Rectangle>),
    /// 타원
    #[serde(rename = "ellipse")]
    Ellipse(Box<Ellipse>),
    /// 호
    #[serde(rename = "arc")]
    Arc(Box<Arc>),
    /// 다각형
    #[serde(rename = "polygon")]
    Polygon(Box<Polygon>),
    /// 곡선
    #[serde(rename = "curve")]
    Curve(Box<Curve>),
    /// 연결선
    #[serde(rename = "connectLine")]
    ConnectLine(Box<ConnectLine>),
    /// 글맵시
    #[serde(rename = "textart")]
    TextArt(Box<TextArt>),
    /// 글자 겹침
    #[serde(rename = "compose")]
    Compose(Compose),
    /// 덧말
    #[serde(rename = "dutmal")]
    Dutmal(Dutmal),
    /// 버튼
    #[serde(rename = "btn")]
    Button(Box<Button>),
    /// 라디오 버튼
    #[serde(rename = "radioBtn")]
    RadioButton(Box<Button>),
    /// 체크 버튼
    #[serde(rename = "checkBtn")]
    CheckButton(Box<Button>),
    /// 콤보 박스
    #[serde(rename = "comboBox")]
    ComboBox(Box<ComboBox>),
    /// 목록 상자
    #[serde(rename = "listBox")]
    ListBox(Box<ListBox>),
    /// 편집 상자
    #[serde(rename = "edit")]
    Edit(Box<Edit>),
    /// 스크롤바
    #[serde(rename = "scrollBar")]
    ScrollBar(Box<ScrollBar>),
    /// 비디오
    #[serde(rename = "video")]
    Video(Box<Video>),
    /// 차트
    #[serde(rename = "chart")]
    Chart(Box<Chart>),
    /// 알 수 없는 개체
    #[serde(rename = "unknownObject")]
    UnknownObject(Box<UnknownObject>),
}

/// [AI 생성] 런
///
/// 원본: `run` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "run")]
pub struct Run {
    /// [AI 생성] 런 내용
    #[serde(rename = "$value", default, skip_serializing_if = "Vec::is_empty")]
    pub contents: Vec<RunContent>,

    /// [AI 생성] 글자 모양 아이디 참조 (`charPrIDRef` 속성)
    #[serde(rename = "@charPrIDRef", skip_serializing_if = "Option::is_none")]
    pub character_property_id_reference: Option<CharShapeIdRef>,

    /// [AI 생성] 글자 변경 추적 아이디 (`charTcId` 속성)
    #[serde(rename = "@charTcId", skip_serializing_if = "Option::is_none")]
    pub character_track_change_id: Option<u32>,
}
