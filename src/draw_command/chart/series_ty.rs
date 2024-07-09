#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChartSeriesLineConfig {
    pub dashed: bool,
}

impl std::default::Default for ChartSeriesLineConfig {
    fn default() -> Self {
        Self { dashed: false }
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChartSeriesScatterConfig {
    pub filled: bool,
}

impl std::default::Default for ChartSeriesScatterConfig {
    fn default() -> Self {
        Self { filled: true }
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ChartSeriesType {
    Line(ChartSeriesLineConfig),
    Scatter(ChartSeriesScatterConfig),
}

impl std::default::Default for ChartSeriesType {
    fn default() -> Self {
        Self::Line(ChartSeriesLineConfig::default())
    }
}

impl ChartSeriesType {
    pub fn get_line(self) -> Option<ChartSeriesLineConfig> {
        match self {
            Self::Line(line) => Some(line),
            _ => None,
        }
    }

    pub fn dashed(self) -> Self {
        let mut line = self.get_line().unwrap_or_default();
        line.dashed = true;
        Self::Line(line)
    }

    pub fn scatter(self) -> Self {
        Self::Scatter(Default::default())
    }

    pub fn scatter_not_filled(self) -> Self {
        Self::Scatter(ChartSeriesScatterConfig { filled: false })
    }
}
