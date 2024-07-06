#[cfg(all(feature = "builder", feature = "plotters"))]
fn main() {
    use plot_commands::draw_command::{plot_chart, plot_layout};

    // create some arbitrary data that will look good on a scatter plot
    let scatter_data = (0..100)
        .map(|i| i as f32 / 100.0)
        .map(|x| (x, x + (x * 100.0).cos() / 10.0))
        .collect::<Vec<_>>();

    plot_layout(|b| {
        b.vsplit([
            plot_chart(|b| {
                b.add_series_l_with(|b| {
                    b.data(scatter_data)
                        .scatter_not_filled()
                        .name("scatter data")
                })
                .title("Top")
            }),
            plot_layout(|b| {
                b.hsplit([
                    plot_chart(|b| {
                        b.title("Bottom Left")
                            .add_series_l(([1, 2, 3], "my-series"))
                    }),
                    plot_chart(|b| {
                        b.title("Bottom Right")
                            .add_series_l(([3, 2, 1], "my-series"))
                    }),
                ])
            }),
        ])
    })
    .plot_png("./subplots.png", (512, 512));
}

#[cfg(not(all(feature = "builder", feature = "plotters")))]
fn main() {
    panic!("This example requires the `builder` and `plotters` features");
}
