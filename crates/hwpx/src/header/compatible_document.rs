//! [AI 생성 문서화] 문서 호환성
//!
//! KS X 6101:2024 - header.xsd 기반 설명입니다. 실제 스키마(`docs/hwpx/schemas/header.xsd`)와 차이가 있으면 TODO로 보완하세요.

use serde::{Deserialize, Serialize};

/// [AI 생성] 대상 프로그램
///
/// 원본: `CompatibleDocumentType.targetProgram` 속성의 익명 타입. HWP/HWPX가 어떤 뷰어/버전에 맞춰졌는지 나타냅니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TargetProgram {
    /// HWP 201X
    #[default]
    #[serde(rename = "HWP201X")]
    Hwp201X,
    /// HWP 200X
    #[serde(rename = "HWP200X")]
    Hwp200X,
    /// MS Word
    #[serde(rename = "MS_WORD")]
    MsWord,
}

/// [AI 생성] 레이아웃 호환성 설정
///
/// 원본: `layoutCompatibility` 요소의 익명 타입. 출력/정렬/격자/표/이미지/각주 등 호환성 플래그 모음입니다.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "layoutCompatibility")]
pub struct LayoutCompatibility {
    /// [AI 생성] 폰트 굵기를 볼드에 적용 (`applyFontWeightToBold`)
    #[serde(
        rename = "applyFontWeightToBold",
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_font_weight_to_bold: Option<()>,

    /// [AI 생성] 내부 밑줄 사용 (`useInnerUnderline`)
    #[serde(rename = "useInnerUnderline", skip_serializing_if = "Option::is_none")]
    pub use_inner_underline: Option<()>,

    /// [AI 생성] 고정 밑줄 너비 (`fixedUnderlineWidth`)
    #[serde(
        rename = "fixedUnderlineWidth",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed_underline_width: Option<()>,

    /// [AI 생성] 밑줄에 취소선 미적용 (`doNotApplyStrikeoutWithUnderline`)
    #[serde(
        rename = "doNotApplyStrikeoutWithUnderline",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_apply_strikeout_with_underline: Option<()>,

    /// [AI 생성] 소문자 취소선 사용 (`useLowercaseStrikeout`)
    #[serde(
        rename = "useLowercaseStrikeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_lowercase_strikeout: Option<()>,

    /// [AI 생성] 줄 높이를 오프셋까지 확장 (`extendLineheightToOffset`)
    #[serde(
        rename = "extendLineheightToOffset",
        skip_serializing_if = "Option::is_none"
    )]
    pub extend_lineheight_to_offset: Option<()>,

    /// [AI 생성] 라틴에 폰트 간격 적용 (`applyFontspaceToLatin`)
    #[serde(
        rename = "applyFontspaceToLatin",
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_fontspace_to_latin: Option<()>,

    /// [AI 생성] 따옴표를 라틴으로 처리 (`treatQuotationAsLatin`)
    #[serde(
        rename = "treatQuotationAsLatin",
        skip_serializing_if = "Option::is_none"
    )]
    pub treat_quotation_as_latin: Option<()>,

    /// [AI 생성] None/Six 기호 표시 미적용 (`doNotApplyDiacSymMarkOfNoneAndSix`)
    #[serde(
        rename = "doNotApplyDiacSymMarkOfNoneAndSix",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_apply_diac_sym_mark_of_none_and_six: Option<()>,

    /// [AI 생성] 오른쪽 공백 정렬 미적용 (`doNotAlignWhitespaceOnRight`)
    #[serde(
        rename = "doNotAlignWhitespaceOnRight",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_align_whitespace_on_right: Option<()>,

    /// [AI 생성] 양쪽 정렬에서 단어 조정 미적용 (`doNotAdjustWordInJustify`)
    #[serde(
        rename = "doNotAdjustWordInJustify",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_adjust_word_in_justify: Option<()>,

    /// [AI 생성] 동아시아 문자 기준 (`baseCharUnitOnEAsian`)
    #[serde(
        rename = "baseCharUnitOnEAsian",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_char_unit_on_east_asian: Option<()>,

    /// [AI 생성] 첫 글자 기준 들여쓰기 (`baseCharUnitOfIndentOnFirstChar`)
    #[serde(
        rename = "baseCharUnitOfIndentOnFirstChar",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_char_unit_of_indent_on_first_char: Option<()>,

    /// [AI 생성] 글꼴에 줄 높이 조정 (`adjustLineheightToFont`)
    #[serde(
        rename = "adjustLineheightToFont",
        skip_serializing_if = "Option::is_none"
    )]
    pub adjust_lineheight_to_font: Option<()>,

    /// [AI 생성] 고정 줄 간격에서 기준선 조정 (`adjustBaselineInFixedLinespacing`)
    #[serde(
        rename = "adjustBaselineInFixedLinespacing",
        skip_serializing_if = "Option::is_none"
    )]
    pub adjust_baseline_in_fixed_linespacing: Option<()>,

    /// [AI 생성] 객체 아래에 이전 간격 적용 (`applyPrevspacingBeneathObject`)
    #[serde(
        rename = "applyPrevspacingBeneathObject",
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_prevspacing_beneath_object: Option<()>,

    /// [AI 생성] 마지막 문단의 다음 간격 적용 (`applyNextspacingOfLastPara`)
    #[serde(
        rename = "applyNextspacingOfLastPara",
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_nextspacing_of_last_para: Option<()>,

    /// [AI 생성] 100% 이상에 최소 적용 (`applyAtLeastToPercent100Pct`)
    #[serde(
        rename = "applyAtLeastToPercent100Pct",
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_at_least_to_percent_100_pct: Option<()>,

    /// [AI 생성] 동아시아-영문 자동 간격 미적용 (`doNotApplyAutoSpaceEAsianEng`)
    #[serde(
        rename = "doNotApplyAutoSpaceEAsianEng",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_apply_auto_space_east_asian_eng: Option<()>,

    /// [AI 생성] 동아시아-숫자 자동 간격 미적용 (`doNotApplyAutoSpaceEAsianNum`)
    #[serde(
        rename = "doNotApplyAutoSpaceEAsianNum",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_apply_auto_space_east_asian_num: Option<()>,

    /// [AI 생성] 문단 테두리 채우기를 간격에 조정 (`adjustParaBorderfillToSpacing`)
    #[serde(
        rename = "adjustParaBorderfillToSpacing",
        skip_serializing_if = "Option::is_none"
    )]
    pub adjust_para_borderfill_to_spacing: Option<()>,

    /// [AI 생성] 동일 테두리의 문단 테두리 채우기 연결 (`connectParaBorderfillOfEqualBorder`)
    #[serde(
        rename = "connectParaBorderfillOfEqualBorder",
        skip_serializing_if = "Option::is_none"
    )]
    pub connect_para_borderfill_of_equal_border: Option<()>,

    /// [AI 생성] 문단 테두리 오프셋을 테두리에 조정 (`adjustParaBorderOffsetWithBorder`)
    #[serde(
        rename = "adjustParaBorderOffsetWithBorder",
        skip_serializing_if = "Option::is_none"
    )]
    pub adjust_para_border_offset_with_border: Option<()>,

    /// [AI 생성] 줄 높이를 문단 테두리 오프셋까지 확장 (`extendLineheightToParaBorderOffset`)
    #[serde(
        rename = "extendLineheightToParaBorderOffset",
        skip_serializing_if = "Option::is_none"
    )]
    pub extend_lineheight_to_para_border_offset: Option<()>,

    /// [AI 생성] 문단 테두리를 바깥쪽에 적용 (`applyParaBorderToOutside`)
    #[serde(
        rename = "applyParaBorderToOutside",
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_para_border_to_outside: Option<()>,

    /// [AI 생성] 최소 열 너비를 1mm로 적용 (`applyMinColumnWidthTo1mm`)
    #[serde(
        rename = "applyMinColumnWidthTo1mm",
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_min_column_width_to_1mm: Option<()>,

    /// [AI 생성] 세그먼트 기준 탭 위치 적용 (`applyTabPosBasedOnSegment`)
    #[serde(
        rename = "applyTabPosBasedOnSegment",
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_tab_pos_based_on_segment: Option<()>,

    /// [AI 생성] 줄 끝에서 탭 중단 (`breakTabOverLine`)
    #[serde(rename = "breakTabOverLine", skip_serializing_if = "Option::is_none")]
    pub break_tab_over_line: Option<()>,

    /// [AI 생성] 줄의 세로 위치 조정 (`adjustVertPosOfLine`)
    #[serde(
        rename = "adjustVertPosOfLine",
        skip_serializing_if = "Option::is_none"
    )]
    pub adjust_vert_pos_of_line: Option<()>,

    /// [AI 생성] 공백 높이 미적용 (`doNotApplyWhiteSpaceHeight`)
    #[serde(
        rename = "doNotApplyWhiteSpaceHeight",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_apply_white_space_height: Option<()>,

    /// [AI 생성] 마지막 마침표 정렬 미적용 (`doNotAlignLastPeriod`)
    #[serde(
        rename = "doNotAlignLastPeriod",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_align_last_period: Option<()>,

    /// [AI 생성] 마지막 금칙 문자 정렬 미적용 (`doNotAlignLastForbidden`)
    #[serde(
        rename = "doNotAlignLastForbidden",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_align_last_forbidden: Option<()>,

    /// [AI 생성] 줄 격자 기준 줄 간격 (`baseLineSpacingOnLineGrid`)
    #[serde(
        rename = "baseLineSpacingOnLineGrid",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_line_spacing_on_line_grid: Option<()>,

    /// [AI 생성] 문자 격자에 문자 간격 적용 (`applyCharSpacingToCharGrid`)
    #[serde(
        rename = "applyCharSpacingToCharGrid",
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_char_spacing_to_char_grid: Option<()>,

    /// [AI 생성] 머리글/바닥글에 격자 미적용 (`doNotApplyGridInHeaderFooter`)
    #[serde(
        rename = "doNotApplyGridInHeaderFooter",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_apply_grid_in_header_footer: Option<()>,

    /// [AI 생성] 각 구역에 확장 머리글/바닥글 적용 (`applyExtendHeaderFooterEachSection`)
    #[serde(
        rename = "applyExtendHeaderFooterEachSection",
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_extend_header_footer_each_section: Option<()>,

    /// [AI 생성] 공간 없을 때 머리글/바닥글 미적용 (`doNotApplyHeaderFooterAtNoSpace`)
    #[serde(
        rename = "doNotApplyHeaderFooterAtNoSpace",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_apply_header_footer_at_no_space: Option<()>,

    /// [AI 생성] 간격 없을 때 열 구분선 미적용 (`doNotApplyColSeparatorAtNoGap`)
    #[serde(
        rename = "doNotApplyColSeparatorAtNoGap",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_apply_col_separator_at_no_gap: Option<()>,

    /// [AI 생성] 줄 간격 없을 때 줄 격자 미적용 (`doNotApplyLinegridAtNoLinespacing`)
    #[serde(
        rename = "doNotApplyLinegridAtNoLinespacing",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_apply_linegrid_at_no_linespacing: Option<()>,

    /// [AI 생성] 이미지 효과 미적용 (`doNotApplyImageEffect`)
    #[serde(
        rename = "doNotApplyImageEffect",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_apply_image_effect: Option<()>,

    /// [AI 생성] 도형 주석 미적용 (`doNotApplyShapeComment`)
    #[serde(
        rename = "doNotApplyShapeComment",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_apply_shape_comment: Option<()>,

    /// [AI 생성] 빈 앵커 줄 조정 미적용 (`doNotAdjustEmptyAnchorLine`)
    #[serde(
        rename = "doNotAdjustEmptyAnchorLine",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_adjust_empty_anchor_line: Option<()>,

    /// [AI 생성] 양쪽 허용 겹침 허용 (`overlapBothAllowOverlap`)
    #[serde(
        rename = "overlapBothAllowOverlap",
        skip_serializing_if = "Option::is_none"
    )]
    pub overlap_both_allow_overlap: Option<()>,

    /// [AI 생성] 앞으로 세로 오프셋 미적용 (`doNotApplyVertOffsetOfForward`)
    #[serde(
        rename = "doNotApplyVertOffsetOfForward",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_apply_vert_offset_of_forward: Option<()>,

    /// [AI 생성] 세로 제한을 페이지 여백까지 확장 (`extendVertLimitToPageMargins`)
    #[serde(
        rename = "extendVertLimitToPageMargins",
        skip_serializing_if = "Option::is_none"
    )]
    pub extend_vert_limit_to_page_margins: Option<()>,

    /// [AI 생성] 표의 앵커 고정 미적용 (`doNotHoldAnchorOfTable`)
    #[serde(
        rename = "doNotHoldAnchorOfTable",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_hold_anchor_of_table: Option<()>,

    /// [AI 생성] 앵커 아래에서 서식 미적용 (`doNotFormattingAtBeneathAnchor`)
    #[serde(
        rename = "doNotFormattingAtBeneathAnchor",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_formatting_at_beneath_anchor: Option<()>,

    /// [AI 생성] 객체의 기준선을 아래로 조정 (`adjustBaselineOfObjectToBottom`)
    #[serde(
        rename = "adjustBaselineOfObjectToBottom",
        skip_serializing_if = "Option::is_none"
    )]
    pub adjust_baseline_of_object_to_bottom: Option<()>,

    /// [AI 생성] 확장 문자 구성 미적용 (`doNotApplyExtensionCharCompose`)
    #[serde(
        rename = "doNotApplyExtensionCharCompose",
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_apply_extension_char_compose: Option<()>,
}

/// [AI 생성] 문서 호환성
///
/// 원본: `CompatibleDocumentType`. 대상 프로그램과 호환 플래그 묶음입니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "compatibleDocument")]
pub struct CompatibleDocument {
    /// [AI 생성] 레이아웃 호환성 설정 (`layoutCompatibility` 요소)
    #[serde(rename = "layoutCompatibility")]
    pub layout_compatibility: LayoutCompatibility,

    /// [AI 생성] 대상 프로그램 (`targetProgram` 속성)
    #[serde(rename = "@targetProgram")]
    pub target_program: TargetProgram,
}
