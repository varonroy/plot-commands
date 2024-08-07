#[cfg(all(feature = "builder", feature = "plotters"))]
fn main() {
    use std::collections::HashMap;

    use plot_commands::draw_command::{plot, plot_chart, plot_layout};

    // create some arbitrary data that will look good on a scatter plot
    let scatter_data = (0..100)
        .map(|i| i as f32 / 100.0)
        .map(|x| (x, x + (x * 100.0).cos() / 10.0))
        .collect::<Vec<_>>();

    let bottom_left_series = [
        ("series 1".to_string(), [1.0, 1.1, 1.2]),
        ("series 2".to_string(), [1.0, 0.9, 0.5]),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

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
                    plot((bottom_left_series, "Bottom left")),
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
