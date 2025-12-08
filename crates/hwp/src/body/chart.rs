//! Chart data parsing.
//!
//! Charts display data in various visual formats like bar charts,
//! line charts, pie charts, etc.
//!
//! Note: Charts in HWP are stored as OLE objects. The actual chart content
//! is in the OLE Compound file's "Contents" or "OOXMLChartContents" stream.
//! HWPTAG_CHART_DATA only contains a 2-byte header.

use crate::error::Result;
use crate::util::ByteReader;

/// Chart type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChartType {
    /// Unknown or unsupported chart type.
    #[default]
    Unknown,
    /// Bar chart.
    Bar,
    /// Line chart.
    Line,
    /// Pie chart.
    Pie,
    /// Area chart.
    Area,
    /// Scatter chart.
    Scatter,
    /// Combined chart.
    Combined,
    /// Radar chart.
    Radar,
}

impl ChartType {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Bar,
            1 => Self::Line,
            2 => Self::Pie,
            3 => Self::Area,
            4 => Self::Scatter,
            5 => Self::Combined,
            6 => Self::Radar,
            _ => Self::Unknown,
        }
    }
}

/// Chart data series.
#[derive(Debug, Clone, Default)]
pub struct ChartSeries {
    /// Series name.
    pub name: String,
    /// Data values.
    pub values: Vec<f64>,
}

/// Chart data in the document.
#[derive(Debug, Clone, Default)]
pub struct ChartData {
    /// Chart type.
    pub chart_type: ChartType,
    /// Chart title.
    pub title: String,
    /// Category labels (X-axis).
    pub categories: Vec<String>,
    /// Data series.
    pub series: Vec<ChartSeries>,
}

impl ChartData {
    /// Parses chart data record from reader.
    ///
    /// Format (per HWP spec - HWPTAG_CHART_DATA is just 2 bytes):
    /// - UINT16: chart data flags/type
    ///
    /// Note: The actual chart content is stored in the OLE object's
    /// "Contents" or "OOXMLChartContents" stream, not in this record.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        // HWPTAG_CHART_DATA only has 2 bytes per spec
        let chart_flags = if reader.remaining() >= 2 {
            reader.read_u16()?
        } else {
            0
        };

        // Extract chart type from flags (if encoded)
        let chart_type = ChartType::from_raw((chart_flags & 0xFF) as u8);

        Ok(Self {
            chart_type,
            title: String::new(),
            categories: Vec::new(),
            series: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_type_from_raw() {
        assert_eq!(ChartType::from_raw(0), ChartType::Bar);
        assert_eq!(ChartType::from_raw(1), ChartType::Line);
        assert_eq!(ChartType::from_raw(2), ChartType::Pie);
        assert_eq!(ChartType::from_raw(255), ChartType::Unknown);
    }

    #[test]
    fn test_chart_series_default() {
        let series = ChartSeries::default();
        assert_eq!(series.name, "");
        assert!(series.values.is_empty());
    }

    #[test]
    fn test_chart_series_values() {
        let mut series = ChartSeries::default();
        series.values.push(10.0);
        series.values.push(20.0);
        series.values.push(30.0);
        assert_eq!(series.values, &[10.0, 20.0, 30.0]);
    }

    #[test]
    fn test_chart_data_default() {
        let chart = ChartData::default();
        assert_eq!(chart.chart_type, ChartType::Unknown);
        assert_eq!(chart.title, "");
        assert!(chart.categories.is_empty());
        assert!(chart.series.is_empty());
    }

    #[test]
    fn test_chart_data_full() {
        let mut chart = ChartData::default();
        chart.chart_type = ChartType::Bar;
        chart.title = "Monthly Sales".to_string();
        chart.categories = vec!["Jan".to_string(), "Feb".to_string(), "Mar".to_string()];

        let mut series = ChartSeries::default();
        series.name = "2024".to_string();
        series.values = vec![100.0, 150.0, 200.0];
        chart.series.push(series);

        assert_eq!(chart.title, "Monthly Sales");
        assert_eq!(chart.categories.len(), 3);
        assert_eq!(chart.series.len(), 1);
    }
}
