use chart::Chart;
use chart_builder::ChartBuilder;
use series::ChartSeries;
use series_builder::ChartSeriesBuilder;

use crate::point_data::IntoData;

use super::*;

pub trait IntoChart {
    fn into_chart(self) -> Chart;
}

impl IntoChart for Chart {
    fn into_chart(self) -> Chart {
        self
    }
}

impl IntoChart for ChartBuilder {
    fn into_chart(self) -> Chart {
        self.build()
    }
}

impl<T: IntoChartSeriesBuilder> IntoChart for T {
    fn into_chart(self) -> Chart {
        ChartBuilder::default()
            .add_series_l(self.into_series_builder())
            .into_chart()
    }
}

pub trait IntoChartSeriesBuilder {
    fn into_series_builder(self) -> ChartSeriesBuilder;
}

impl IntoChartSeriesBuilder for ChartSeries {
    fn into_series_builder(self) -> ChartSeriesBuilder {
        ChartSeriesBuilder::default().series(self)
    }
}

impl IntoChartSeriesBuilder for ChartSeriesBuilder {
    fn into_series_builder(self) -> ChartSeriesBuilder {
        self
    }
}

impl<T: IntoData> IntoChartSeriesBuilder for T {
    fn into_series_builder(self) -> ChartSeriesBuilder {
        ChartSeriesBuilder::default().data(self.into_data())
    }
}

impl<T: IntoData, S: ToString> IntoChartSeriesBuilder for (T, S) {
    fn into_series_builder(self) -> ChartSeriesBuilder {
        ChartSeriesBuilder::default()
            .data(self.0.into_data())
            .name(self.1)
    }
}
