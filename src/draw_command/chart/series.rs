use crate::{data_bound::DataBound2D, map_reduce::MapReduce};

use super::{series_style::SeriesStyle, series_ty::ChartSeriesType};

#[derive(Debug, Clone)]
pub struct ChartSeries {
    pub name: String,
    pub data: Vec<(f32, f32)>,
    pub chart_series_type: ChartSeriesType,
    pub style: SeriesStyle,
}

impl ChartSeries {
    pub fn bounds(&self) -> Option<DataBound2D> {
        self.data
            .iter()
            .copied()
            .map_reduce(
                |item| DataBound2D::from(item),
                |bounds2d, (x, y)| bounds2d.extend_to_include_bound(x, y),
            )
            .map(|bounds| bounds.into())
    }
}
