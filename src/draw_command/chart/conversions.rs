use std::collections::HashMap;

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

// TODO: use a generic `IntoChartComponent` and use a macro to extend this to tuples of
// many sizes
impl<T: IntoData, S: ToString, ST: ToString> IntoChart for (HashMap<S, T>, ST) {
    fn into_chart(self) -> Chart {
        let mut c = self.0.into_chart();
        c.title = self.1.to_string();
        c
    }
}

impl<T: IntoData, S: ToString> IntoChart for HashMap<S, T> {
    fn into_chart(self) -> Chart {
        self.into_iter()
            .fold(ChartBuilder::default(), |acc, (name, data)| {
                acc.add_series_l((data, name))
            })
            .build()
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

// TODO: use a generic `ChartSeriesBuilderComponent` and use a macro to extend this to tuples of
// many sizes
impl<T: IntoData, S: ToString> IntoChartSeriesBuilder for (T, S) {
    fn into_series_builder(self) -> ChartSeriesBuilder {
        ChartSeriesBuilder::default()
            .data(self.0.into_data())
            .name(self.1)
    }
}
