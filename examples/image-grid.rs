#[cfg(all(feature = "builder", feature = "plotters"))]
fn main() {
    use itertools::Itertools;
    use plot_commands::draw_command::{plot_image_with, plot_layout};

    fn gen_image(seed: u32) -> Vec<Vec<f32>> {
        let s = seed as f32;
        let mut i = 0.0;
        let mut f = || {
            i += 0.5;
            ((s + i).cos() + 1.0) * 0.5
        };

        vec![
            vec![f(), f(), f()],
            vec![f(), f(), f()],
            vec![f(), f(), f()],
        ]
    }

    fn gen_image_rgb(seed: u32) -> Vec<Vec<[f32; 3]>> {
        let mut i = 0;
        let mut f = || {
            i += 1;
            let val = (seed * 27 + i * 13) % 67;
            val as f32 / 67.0
        };

        let rows = 10 + seed;
        let cols = 10 + seed;
        (0..rows)
            .map(|_| (0..cols).map(|_| [f(), f(), f()]).collect_vec())
            .collect_vec()
    }

    fn gen_image_buf(seed: u32) -> image::ImageBuffer<image::Rgb<f32>, Vec<f32>> {
        let rows = 10 + seed;
        let cols = 50 + seed;
        let img = image::ImageBuffer::from_fn(rows, rows, |x, y| {
            let r = (x + y) as f32 / ((rows + cols) as f32);
            let x = x + 5;
            let g = (x + y) as f32 / ((rows + cols) as f32);
            let y = y + 5;
            let b = (x + y) as f32 / ((rows + cols) as f32);
            image::Rgb([r, g, b])
        });
        img
    }

    plot_layout(|b| {
        b.grid_with_rows(
            [
                plot_image_with(gen_image_buf(0), |b| b.axes()),
                plot_image_with(gen_image(1), |b| b.axes()),
                plot_image_with(gen_image(2), |b| b.axes()),
                plot_image_with(gen_image_rgb(0), |b| b.axes()),
                plot_image_with(gen_image_rgb(1), |b| b.axes()),
                plot_image_with(gen_image_rgb(2), |b| b.axes()),
            ],
            3,
        )
    })
    .plot_png("./image-grid.png", (512, 512));
}

#[cfg(not(all(feature = "builder", feature = "plotters")))]
fn main() {
    panic!("This example requires the `builder` and `plotters` features");
}
