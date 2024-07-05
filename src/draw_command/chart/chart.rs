use crate::data_bound::DataBound;

use super::series::ChartSeries;

#[derive(Debug, Clone)]
pub struct Chart {
    /// Primary series, associated with the left y-axis.
    pub series_l: Vec<ChartSeries>,
    /// Secondary series, associated with the right y-axis.
    pub series_r: Vec<ChartSeries>,
    pub x_label: String,
    pub y_label_l: String,
    pub y_label_r: String,
    pub title: String,
    pub margin: i32,
}

impl Chart {
    /// Returns the bounds of the chart's data: the x bounds, y, bounds and secondary y bounds.
    pub fn bounds(&self) -> (Option<DataBound>, Option<DataBound>, Option<DataBound>) {
        let l = self
            .series_l
            .iter()
            .filter_map(|seires| seires.bounds())
            .reduce(|acc, x| acc.max(x));
        let lx = l.map(|b| b.x);
        let ly = l.map(|b| b.y);

        let r = self
            .series_r
            .iter()
            .filter_map(|seires| seires.bounds())
            .reduce(|acc, x| acc.extend_to_include_bound2d(x));

        let rx = r.map(|b| b.x);
        let ry = r.map(|b| b.y);

        let x = [lx, rx]
            .into_iter()
            .filter_map(|x| x)
            .reduce(DataBound::extend_to_include);

        (x, ly, ry)
    }
}
