#[derive(Debug, Clone, Copy)]
pub struct ChartSeriesLineConfig {
    pub dashed: bool,
}

impl std::default::Default for ChartSeriesLineConfig {
    fn default() -> Self {
        Self { dashed: false }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ChartSeriesType {
    Line(ChartSeriesLineConfig),
    Scatter,
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
}
