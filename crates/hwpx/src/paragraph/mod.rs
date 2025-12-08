//! [AI 생성 문서화] 문단 모듈
//!
//! 문단 본문, 런, 도형, 표, 양식 컨트롤까지 한 번에 담는 집합입니다. KS X 6101:2024 `paralist.xsd`를 근거로 했으니, 실제 문서 규격(지면/단/텍스트 흐름)과 대조해 검증해 주세요.

pub mod column;
pub mod control;
pub mod drawing;
pub mod effects;
pub mod enums;
pub mod form_control;
pub mod line_shape;
pub mod ole_equation;
pub mod para;
pub mod para_list;
pub mod picture;
pub mod run;
pub mod section_definition;
pub mod shadow;
pub mod shape_common;
pub mod table;
pub mod text;
pub mod text_art;
pub mod video_chart;

pub use column::*;
pub use control::*;
pub use drawing::*;
pub use effects::*;
pub use enums::*;
pub use form_control::*;
pub use line_shape::*;
pub use ole_equation::*;
pub use para::*;
pub use para_list::*;
pub use picture::*;
pub use run::*;
pub use section_definition::*;
pub use shadow::*;
pub use shape_common::*;
pub use table::*;
pub use text::*;
pub use text_art::*;
pub use video_chart::*;
