#[cfg(all(feature = "builder", feature = "plotters"))]
fn main() {
    use plot_commands::draw_command::{plot, plot_chart};

    plot([1, 2, 3]);
    plot([(1, 1), (2, 2)]);

    plot_chart(|b| {
        b.add_series_l(([1, 3, 0], "left"))
            .add_series_l([0, 3, 1])
            .add_series_r(([1, 2, 3], "right"))
            .add_series_r_with(|b| {
                b.data([10.0, 0.1, 0.0, -1.0, 1.0])
                    .name("custom series")
                    .dashed()
            })
            .title("my title")
            .x_label("x")
            .y_label_l("y left")
            .y_label_r("y right")
    })
    .plot_png("./simple_plot.png", (512, 512));
}

#[cfg(not(all(feature = "builder", feature = "plotters")))]
fn main() {
    panic!("This example requires the `builder` and `plotters` features");
}
