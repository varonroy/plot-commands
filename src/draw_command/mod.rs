pub mod chart;
pub mod image;
pub mod layout;

use self::image::Image;
use chart::chart::Chart;
use derive_more::From;
use layout::Layout;

#[derive(Debug, Clone, From)]
pub enum DrawComand {
    Blank,
    Chart(Box<Chart>),
    Image(Box<Image>),
    Layout(Box<Layout>),
}

impl From<Chart> for DrawComand {
    fn from(value: Chart) -> Self {
        Self::Chart(Box::new(value))
    }
}

impl From<Layout> for DrawComand {
    fn from(value: Layout) -> Self {
        Self::Layout(Box::new(value))
    }
}

#[cfg(feature = "plotters")]
pub mod with_plotters {
    use super::DrawComand;
    use std::path::PathBuf;

    impl DrawComand {
        pub fn plot_png(self, path: impl Into<PathBuf>, (w, h): (u32, u32)) {
            use crate::cmd_processor::CmdProcessor;

            let processor = crate::cmd_processor::plotters::PlottersProcessor {
                path: path.into(),
                w,
                h,
            };
            processor.proces(&self);
        }
    }
}

#[cfg(feature = "builder")]
mod conversions {
    use super::{
        chart::{chart_builder::ChartBuilder, IntoChart},
        layout::layout_builder::LayoutBuilder,
        DrawComand,
    };

    pub trait IntoDrawCommand {
        fn into_draw_command(self) -> DrawComand;
    }

    impl IntoDrawCommand for DrawComand {
        fn into_draw_command(self) -> DrawComand {
            self
        }
    }

    impl<T: IntoChart> IntoDrawCommand for T {
        fn into_draw_command(self) -> DrawComand {
            DrawComand::from(self.into_chart())
        }
    }

    impl IntoDrawCommand for LayoutBuilder {
        fn into_draw_command(self) -> DrawComand {
            DrawComand::from(self.build())
        }
    }

    pub fn plot(cmd: impl IntoDrawCommand) -> DrawComand {
        cmd.into_draw_command()
    }

    pub fn plot_image(image: impl Into<super::image::Image>) -> DrawComand {
        DrawComand::Image(Box::new(image.into()))
    }

    pub fn plot_chart(f: impl FnOnce(ChartBuilder) -> ChartBuilder) -> DrawComand {
        let b = ChartBuilder::default();
        let b = f(b);
        plot(b.build())
    }

    pub fn plot_layout(f: impl FnOnce(LayoutBuilder) -> LayoutBuilder) -> DrawComand {
        let b = LayoutBuilder::default();
        let b = f(b);
        plot(b)
    }
}

#[cfg(feature = "builder")]
pub use conversions::*;
