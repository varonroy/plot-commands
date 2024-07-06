use image::imageops::FilterType;
use rgb::RGBA;
use std::path::PathBuf;

use itertools::{chain, izip};
use plotters::{
    backend::{BitMapBackend, DrawingBackend},
    chart::{ChartBuilder, ChartContext, LabelAreaPosition, SeriesAnno, SeriesLabelStyle},
    coord::{types::RangedCoordf32, CoordTranslate, Shift},
    drawing::{DrawingArea, IntoDrawingArea},
    element::{BitMapElement, Circle, PathElement},
    prelude::Cartesian2d,
    series::{DashedLineSeries, LineSeries},
    style::{Color, RGBAColor, ShapeStyle},
};

use crate::{
    data_bound::DataBound,
    draw_command::{
        chart::{chart::Chart, series::ChartSeries},
        layout::Layout,
        DrawComand,
    },
};

use super::CmdProcessor;

fn convert_color(RGBA { r, g, b, a }: RGBA<f32>) -> RGBAColor {
    RGBAColor(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        a as _,
    )
}

fn convert_style(style: crate::draw_command::chart::series_style::SeriesStyle) -> ShapeStyle {
    ShapeStyle {
        color: convert_color(style.color),
        filled: false,
        stroke_width: 1,
    }
}

pub struct PlottersProcessor {
    pub path: PathBuf,
    pub w: u32,
    pub h: u32,
}

impl PlottersProcessor {
    fn configure_series(&self, s: &mut SeriesAnno<impl DrawingBackend>, series: &ChartSeries) {
        s.label(&series.name);
        if !series.name.is_empty() {
            let color = convert_color(series.style.color);
            s.label(&series.name)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }
    }

    fn add_series(
        &self,
        chart: &mut ChartContext<impl DrawingBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
        series: &ChartSeries,
    ) {
        use crate::draw_command::chart::series_ty::ChartSeriesType as CST;
        match series.chart_series_type {
            CST::Line(line) => {
                if line.dashed {
                    let s = DashedLineSeries::new(
                        series.data.clone(),
                        10,
                        8,
                        convert_style(series.style),
                    );
                    let s = chart.draw_series(s).unwrap();
                    self.configure_series(s, series);
                } else {
                    let style = series.style;
                    // TODO: configure stroke width
                    let s =
                        LineSeries::new(series.data.clone(), convert_style(style).stroke_width(1));
                    let s = chart.draw_series(s).unwrap();
                    self.configure_series(s, series);
                }
            }
            CST::Scatter(scatter) => {
                let color = convert_color(series.style.color);
                let color = if scatter.filled {
                    color.filled()
                } else {
                    color.into()
                };

                let s = chart
                    .draw_series(
                        series
                            .data
                            .iter()
                            .map(|(x, y)| Circle::new((*x, *y), 2, color)),
                    )
                    .unwrap();
                self.configure_series(s, series);
            }
        };
    }

    fn configure_series_labels<'a, 'b>(
        &self,
        series_labels: &mut SeriesLabelStyle<'a, 'b, impl DrawingBackend + 'a, impl CoordTranslate>,
        chart_cmd: &Chart,
    ) {
        if chain!(&chart_cmd.series_l, &chart_cmd.series_r).any(|series| !series.name.is_empty()) {
            series_labels
                .position(plotters::chart::SeriesLabelPosition::UpperLeft)
                .background_style(&plotters::style::WHITE.mix(0.8))
                .border_style(&plotters::style::BLACK);
        }
    }

    fn process_chart(&self, area: &DrawingArea<impl DrawingBackend, Shift>, chart_cmd: &Chart) {
        let (x_bounds, y_bounds, y_bounds_r) = chart_cmd.bounds();
        let x_range = x_bounds.unwrap_or(DataBound::zero()).as_range();
        let y_range = y_bounds.unwrap_or(DataBound::zero()).as_range();
        let y_range_r = y_bounds_r.unwrap_or(DataBound::zero()).as_range();

        let m = 40;

        let right_margin = if chart_cmd.series_r.is_empty() { 0 } else { m };

        let mut chart = ChartBuilder::on(area)
            .caption(&chart_cmd.title, ("sans-serif", 20))
            .margin(0)
            .margin_right(m - right_margin)
            .x_label_area_size(m)
            .y_label_area_size(m)
            .right_y_label_area_size(right_margin)
            .build_cartesian_2d(x_range.clone(), y_range)
            .unwrap()
            .set_secondary_coord(x_range, y_range_r);

        chart
            .configure_mesh()
            .x_max_light_lines(1)
            .y_max_light_lines(1)
            .y_desc(&chart_cmd.y_label_l)
            .draw()
            .unwrap();

        for series in chart_cmd.series_l.clone() {
            self.add_series(&mut chart, &series);
        }

        if chart_cmd.series_r.is_empty() {
            let mut series_labels = chart.configure_series_labels();
            self.configure_series_labels(&mut series_labels, chart_cmd);
            series_labels.draw().unwrap();
        } else {
            chart
                .configure_secondary_axes()
                .y_desc(&chart_cmd.y_label_r)
                .draw()
                .unwrap();

            for series in chart_cmd.series_r.clone() {
                self.add_series(&mut chart, &series);
            }

            let mut series_labels = chart.configure_series_labels();
            self.configure_series_labels(&mut series_labels, chart_cmd);
            series_labels.draw().unwrap();
        }
    }

    fn process_image(
        &self,
        area: &DrawingArea<impl DrawingBackend, Shift>,
        image: &crate::draw_command::image::Image,
    ) {
        let mut chart = ChartBuilder::on(&area);
        if !image.title.is_empty() {
            chart.caption("Bitmap Example", ("sans-serif", 20));
        }
        let mut chart = chart
            .margin(5)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(0.0..1.0, 0.0..1.0)
            .unwrap();

        chart.configure_mesh().disable_mesh().draw().unwrap();

        let (w, h) = chart.plotting_area().dim_in_pixel();
        let image = image.to_image_dynamic_image().resize_exact(
            w - w / 10,
            h - h / 10,
            FilterType::Nearest,
        );

        let elem: BitMapElement<_> = ((0.05, 0.95), image).into();

        chart.draw_series(std::iter::once(elem)).unwrap();
    }

    fn process_layout(&self, area: &DrawingArea<impl DrawingBackend, Shift>, layout: &Layout) {
        use crate::draw_command::layout::Layout;
        match layout {
            Layout::Box(cmd) => {
                self.process_command(&area, cmd);
            }
            Layout::VSplit(cmds) => {
                let areas = area.split_evenly((cmds.len(), 1));
                for (area, cmd) in izip!(areas, cmds) {
                    self.process_command(&area, cmd);
                }
            }
            Layout::HSplit(cmds) => {
                let areas = area.split_evenly((1, cmds.len()));
                for (area, cmd) in izip!(areas, cmds) {
                    self.process_command(&area, cmd);
                }
            }
            Layout::Grid {
                commands: cmds,
                constraint,
            } => {
                let (rows, cols) = constraint.calculate_rows_cols(cmds.len());

                let areas = area.split_evenly((rows, cols));
                for (area, cmd) in izip!(areas, cmds) {
                    self.process_command(&area, cmd);
                }
            }
        }
    }

    fn process_command(&self, area: &DrawingArea<impl DrawingBackend, Shift>, cmd: &DrawComand) {
        match cmd {
            DrawComand::Blank => {}
            DrawComand::Chart(chart) => self.process_chart(area, chart),
            DrawComand::Image(image) => self.process_image(area, &image),
            DrawComand::Layout(layout) => self.process_layout(area, layout),
        }
    }
}

impl CmdProcessor for PlottersProcessor {
    fn proces(&self, cmd: &DrawComand) {
        let backend = BitMapBackend::new(&self.path, (self.w, self.h));
        let root = backend.into_drawing_area();
        root.fill(&plotters::style::WHITE).unwrap();
        self.process_command(&root, cmd);
        root.present().unwrap();
    }
}
