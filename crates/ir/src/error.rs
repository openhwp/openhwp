//! 변환 에러 및 경고 처리
//!
//! 문서 변환 시 발생하는 에러와 경고를 정의합니다.

use std::fmt;

/// 변환 에러 종류
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversionErrorKind {
    /// 지원하지 않는 형식
    UnsupportedFormat,
    /// 잘못된 데이터
    InvalidData,
    /// 필수 데이터 누락
    MissingRequired,
    /// 참조 오류 (존재하지 않는 ID 참조)
    InvalidReference,
    /// 내부 오류
    Internal,
}

/// 변환 에러
#[derive(Debug, Clone)]
pub struct ConversionError {
    /// 에러 종류
    pub kind: ConversionErrorKind,
    /// 에러 메시지
    pub message: String,
    /// 에러 발생 위치 (선택적)
    pub location: Option<String>,
}

impl ConversionError {
    /// 에러 생성
    pub fn new(kind: ConversionErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            location: None,
        }
    }

    /// 위치 정보 추가
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// 지원하지 않는 형식 에러
    pub fn unsupported(message: impl Into<String>) -> Self {
        Self::new(ConversionErrorKind::UnsupportedFormat, message)
    }

    /// 잘못된 데이터 에러
    pub fn invalid_data(message: impl Into<String>) -> Self {
        Self::new(ConversionErrorKind::InvalidData, message)
    }

    /// 필수 데이터 누락 에러
    pub fn missing_required(message: impl Into<String>) -> Self {
        Self::new(ConversionErrorKind::MissingRequired, message)
    }

    /// 참조 오류
    pub fn invalid_reference(message: impl Into<String>) -> Self {
        Self::new(ConversionErrorKind::InvalidReference, message)
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref loc) = self.location {
            write!(f, "[{}] {}", loc, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for ConversionError {}

/// 변환 경고 종류
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversionWarningKind {
    /// 데이터 손실 (대상 형식에서 지원하지 않는 기능)
    DataLoss,
    /// 기본값으로 대체됨
    DefaultSubstituted,
    /// 근사값으로 변환됨
    Approximated,
    /// 알 수 없는 값 무시됨
    UnknownIgnored,
}

/// 변환 경고
#[derive(Debug, Clone)]
pub struct ConversionWarning {
    /// 경고 종류
    pub kind: ConversionWarningKind,
    /// 경고 메시지
    pub message: String,
    /// 경고 발생 위치 (선택적)
    pub location: Option<String>,
}

impl ConversionWarning {
    /// 경고 생성
    pub fn new(kind: ConversionWarningKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            location: None,
        }
    }

    /// 위치 정보 추가
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// 데이터 손실 경고
    pub fn data_loss(message: impl Into<String>) -> Self {
        Self::new(ConversionWarningKind::DataLoss, message)
    }

    /// 기본값 대체 경고
    pub fn default_substituted(message: impl Into<String>) -> Self {
        Self::new(ConversionWarningKind::DefaultSubstituted, message)
    }

    /// 근사값 변환 경고
    pub fn approximated(message: impl Into<String>) -> Self {
        Self::new(ConversionWarningKind::Approximated, message)
    }

    /// 알 수 없는 값 무시 경고
    pub fn unknown_ignored(message: impl Into<String>) -> Self {
        Self::new(ConversionWarningKind::UnknownIgnored, message)
    }
}

impl fmt::Display for ConversionWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref loc) = self.location {
            write!(f, "[{}] {}", loc, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

/// 변환 결과
///
/// 변환된 값과 경고를 함께 반환합니다.
#[derive(Debug, Clone)]
pub struct ConversionResult<T> {
    /// 변환된 값
    pub value: T,
    /// 변환 중 발생한 경고들
    pub warnings: Vec<ConversionWarning>,
}

impl<T> ConversionResult<T> {
    /// 경고 없이 결과 생성
    pub fn ok(value: T) -> Self {
        Self {
            value,
            warnings: Vec::new(),
        }
    }

    /// 경고와 함께 결과 생성
    pub fn with_warnings(value: T, warnings: Vec<ConversionWarning>) -> Self {
        Self { value, warnings }
    }

    /// 단일 경고 추가
    pub fn with_warning(mut self, warning: ConversionWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// 값 변환
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> ConversionResult<U> {
        ConversionResult {
            value: f(self.value),
            warnings: self.warnings,
        }
    }

    /// 경고 유무 확인
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// 값과 경고 분리
    pub fn into_parts(self) -> (T, Vec<ConversionWarning>) {
        (self.value, self.warnings)
    }
}

impl<T: Default> Default for ConversionResult<T> {
    fn default() -> Self {
        Self::ok(T::default())
    }
}

/// 경고 수집기
///
/// 변환 과정에서 경고를 수집하는 헬퍼입니다.
#[derive(Debug, Default)]
pub struct WarningCollector {
    warnings: Vec<ConversionWarning>,
}

impl WarningCollector {
    /// 새 수집기 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 경고 추가
    pub fn push(&mut self, warning: ConversionWarning) {
        self.warnings.push(warning);
    }

    /// 데이터 손실 경고 추가
    pub fn data_loss(&mut self, message: impl Into<String>) {
        self.push(ConversionWarning::data_loss(message));
    }

    /// 기본값 대체 경고 추가
    pub fn default_substituted(&mut self, message: impl Into<String>) {
        self.push(ConversionWarning::default_substituted(message));
    }

    /// 다른 수집기의 경고들을 병합
    pub fn merge(&mut self, other: WarningCollector) {
        self.warnings.extend(other.warnings);
    }

    /// 다른 결과의 경고들을 병합
    pub fn merge_from<T>(&mut self, result: &ConversionResult<T>) {
        self.warnings.extend(result.warnings.iter().cloned());
    }

    /// 결과와 함께 반환
    pub fn into_result<T>(self, value: T) -> ConversionResult<T> {
        ConversionResult::with_warnings(value, self.warnings)
    }

    /// 경고 목록 반환
    pub fn into_warnings(self) -> Vec<ConversionWarning> {
        self.warnings
    }

    /// 경고 유무 확인
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_error() {
        let err = ConversionError::unsupported("테스트").with_location("Section[0]");
        assert_eq!(err.kind, ConversionErrorKind::UnsupportedFormat);
        assert!(err.to_string().contains("Section[0]"));
    }

    #[test]
    fn test_conversion_result() {
        let result = ConversionResult::ok(42).with_warning(ConversionWarning::data_loss("test"));

        assert_eq!(result.value, 42);
        assert!(result.has_warnings());
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_warning_collector() {
        let mut collector = WarningCollector::new();
        collector.data_loss("field A 손실");
        collector.default_substituted("field B 기본값 사용");

        let result: ConversionResult<i32> = collector.into_result(100);
        assert_eq!(result.warnings.len(), 2);
    }
}
