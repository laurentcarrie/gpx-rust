use argh::FromArgs;
use lc_gpx_utils::transform::derivate::add_derivate_column;
use log::LevelFilter;
use plotlars::{LinePlot, Plot};
use polars::prelude::Series;
use polars::prelude::*;

#[derive(Debug, FromArgs)]
#[argh(description = "generate html plot from garmin tcx file")]
struct Cli {
    // #[argh(positional, description = "garmin tcx file to read")]
    // tcx_path: String,
    #[argh(positional, description = "output html file")]
    html_path: String,
}

fn main() {
    let _ = simple_logging::log_to_file("/tmp/foo.log", LevelFilter::Info)
        .expect("Failed to initialize logging");
    log::info!("start test plot derivative");
    let cli: Cli = argh::from_env();

    let imax = 1000;
    let vt = (0..imax).map(|t| t as f64).collect::<Vec<_>>();
    let vy = (0..imax).map(|t| t as f64 * t as f64).collect::<Vec<_>>();
    let mut df = DataFrame::new(vec![
        Series::new("time".into(), vt).into(),
        Series::new("y".into(), vy).into(),
    ])
    .unwrap();

    let df = add_derivate_column(&mut df, "y", "dy/dx").expect("derivative");

    let _ = LinePlot::builder()
        .data(&df)
        .x("t")
        .y("y")
        .additional_lines(vec!["s"])
        .size(12)
        .plot_title("1st order")
        .x_title("time")
        .y_title("x")
        .legend_title("Allure")
        .build()
        .write_html(cli.html_path);

    log::info!("{}:{} end of test", file!(), line!());
}
