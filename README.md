# Plot Commands
The goal of this library is to provide a declarative way to define plots. The format provided by this crate combines both the data that is plotted and the styling / visualization description. Using this format has a few advantages over other plotting methods:
- When a plot is shared as an image, all of its data is lost, and it can no longer be opened in an interactive viewer.
- When a plot is shared as a data format, the visual / presentation data is lost, and it can only be visualized using an accompanying code.

## Running Examples
Some of the examples require various features. To enable them, add `--features="<space-separated features>"`. For example:
```
cargo run --example simple-plot --features="builder plotters"
```
For simplicity, if the `all` feature is specified, all the examples can be ran without specifying specific dependencies.

```
cargo run --example simple-plot --features=all
```
