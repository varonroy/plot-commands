use crate::point_data::IntoData;
use rgb::RGBA;

use super::series::ChartSeries;

pub struct ChartSeriesBuilder {
    pub s: ChartSeries,
    pub auto_color: bool,
}

impl std::default::Default for ChartSeriesBuilder {
    fn default() -> Self {
        Self {
            s: ChartSeries {
                name: "".to_string(),
                data: Vec::new(),
                chart_series_type: Default::default(),
                style: Default::default(),
            },
            auto_color: true,
        }
    }
}

impl ChartSeriesBuilder {
    pub fn series(mut self, series: impl Into<ChartSeries>) -> Self {
        self.s = series.into();
        self.auto_color = false;
        self
    }

    pub fn name(mut self, name: impl ToString) -> Self {
        self.s.name = name.to_string();
        self
    }

    pub fn data(mut self, data: impl IntoData) -> Self {
        self.s.data = data.into_data();
        self
    }

    pub fn auto_color(mut self) -> Self {
        self.auto_color = true;
        self
    }

    pub fn color(mut self, color: impl Into<RGBA<f32>>) -> Self {
        self.s.style.color = color.into();
        self.auto_color = false;
        self
    }

    pub fn dashed(mut self) -> Self {
        self.s.chart_series_type = self.s.chart_series_type.dashed();
        self
    }

    pub fn build(self) -> ChartSeries {
        self.s
    }

    pub fn build_with_color(self, color: impl Into<RGBA<f32>>) -> ChartSeries {
        let mut s = self.s;
        if self.auto_color {
            s.style.color = color.into();
        }
        s
    }
}
