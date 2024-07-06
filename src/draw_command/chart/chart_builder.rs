use crate::colors::Palette;

use super::{chart::Chart, series_builder::ChartSeriesBuilder, IntoChartSeriesBuilder};

pub struct ChartBuilder {
    c: Chart,
    p: Palette,
    series_l: Vec<ChartSeriesBuilder>,
    series_r: Vec<ChartSeriesBuilder>,
}

impl std::default::Default for ChartBuilder {
    fn default() -> Self {
        Self {
            c: Chart {
                series_l: vec![],
                series_r: vec![],
                x_label: "".to_string(),
                y_label_l: "".to_string(),
                y_label_r: "".to_string(),
                title: "".to_string(),
                margin: 40,
            },
            p: Default::default(),
            series_l: vec![],
            series_r: vec![],
        }
    }
}

impl ChartBuilder {
    pub fn add_series_l(mut self, series: impl IntoChartSeriesBuilder) -> Self {
        self.series_l.push(series.into_series_builder());
        self
    }

    pub fn add_series_l_with(
        self,
        f: impl FnOnce(ChartSeriesBuilder) -> ChartSeriesBuilder,
    ) -> Self {
        let csb = ChartSeriesBuilder::default();
        self.add_series_l(f(csb))
    }

    pub fn add_series_r(mut self, series: impl IntoChartSeriesBuilder) -> Self {
        self.series_r.push(series.into_series_builder());
        self
    }

    pub fn add_series_r_with(
        self,
        f: impl FnOnce(ChartSeriesBuilder) -> ChartSeriesBuilder,
    ) -> Self {
        let csb = ChartSeriesBuilder::default();
        self.add_series_r(f(csb))
    }

    pub fn x_label(mut self, label: impl ToString) -> Self {
        self.c.x_label = label.to_string();
        self
    }

    pub fn y_label_l(mut self, label: impl ToString) -> Self {
        self.c.y_label_l = label.to_string();
        self
    }

    pub fn y_label_r(mut self, label: impl ToString) -> Self {
        self.c.y_label_r = label.to_string();
        self
    }

    pub fn title(mut self, title: impl ToString) -> Self {
        self.c.title = title.to_string();
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.c.margin = margin;
        self
    }

    pub fn build(self) -> Chart {
        let mut chart = self.c;

        let mut colors = self.p.iter();
        for s in self.series_l {
            let s = if let Some(c) = colors.next() {
                s.build_with_color(c)
            } else {
                s.build()
            };
            chart.series_l.push(s);
        }

        for s in self.series_r {
            let s = if let Some(c) = colors.next() {
                s.build_with_color(c)
            } else {
                s.build()
            };
            chart.series_r.push(s);
        }

        chart
    }
}
