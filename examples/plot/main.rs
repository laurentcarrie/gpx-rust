use std::fs::File;

use argh::FromArgs;
use lc_gpx_utils::tcx::read::df_of_tcx_file;
use lc_gpx_utils::transform::filter::add_filter_columns;
use lc_gpx_utils::transform::kml_of_df::circuits_of_df;
use log::LevelFilter;
use plotlars::{Plot, Rgb, TimeSeriesPlot};
use polars::prelude::*;

#[derive(Debug, FromArgs)]
#[argh(description = "generate html plot from garmin tcx file")]
struct Cli {
    #[argh(positional, description = "garmin tcx file to read")]
    tcx_path: String,
    #[argh(positional, description = "output html file")]
    html_path: String,
}

fn main() {
    let _ = simple_logging::log_to_file("/tmp/foo.log", LevelFilter::Info)
        .expect("Failed to initialize logging");
    log::info!("start test plot");
    let cli: Cli = argh::from_env();

    let df = df_of_tcx_file(cli.tcx_path).expect("should get dataframe from tcx file");
    let max_allure = 10.0;
    let mut df = df
        .lazy()
        .with_column(
            col("time")
                .cast(DataType::Datetime(
                    TimeUnit::Milliseconds,
                    Some(TimeZone::UTC),
                ))
                .alias("t"),
        )
        .with_column(
            when(col("allure").gt(max_allure))
                .then(lit(max_allure))
                .otherwise(col("allure"))
                .alias("capped_allure"),
        )
        .with_column(
            when(col("capped_allure").lt(0.0))
                .then(lit(0.0))
                .otherwise(col("capped_allure"))
                .alias("capped_allure2"),
        )
        .collect()
        .expect("cap allure");

    let circuits = circuits_of_df(&df).expect("circuits");
    for c in &circuits {
        df = df
            .lazy()
            .with_column(
                when(col("circuit").eq(lit(*c)))
                    .then(col("capped_allure2"))
                    .otherwise(lit(NULL))
                    .alias(format!("c_{}", c)),
            )
            .collect()
            .expect("add circuit");
    }
    // df = df.).expect("collect");
    // let df = add_filter_columns(&mut df, "speed", "speed_f", 1.5, 1.0).expect("add filter columns");
    let df =
        add_filter_columns(&mut df, "allure", "allure_f", 1.5, 1.0).expect("add filter columns");
    // log::info!("{}:{} {:?}", file!(), line!(), df.schema());

    let df = df
        .lazy()
        .with_column((col("distance") / col("time") * lit(3600)).alias("speedx"))
        .collect()
        .expect("collect");

    log::info!("{}:{} {:?}", file!(), line!(), df.schema());

    // log::info!(
    //     "{:}",
    //     &df.clone().lazy()
    //         .select(vec![col("time"), col("capped_allure"), col("allure")])
    //         .collect()
    //         .unwrap()
    // );

    {
        let mut file = File::create("example.csv").expect("could not create file");
        let mut df2 = (&df).clone();
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut df2)
            .expect("write csv");
    }
    // let other_circuits = circuits[1..circuits.len()]
    //     .iter()
    //     .map(|c| format!("c_{}", c))
    //     .collect::<Vec<_>>();
    // let mut other_circuits_str: Vec<&str> = other_circuits.iter().map(|s| s.as_str()).collect();
    // let mut other_circuits_str: Vec<&str> = vec!["distance"];
    let _ = TimeSeriesPlot::builder()
        .data(&df)
        .x("t")
        .y("speedx")
        .additional_series(vec!["allure_f"])
        .size(12)
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .plot_title("Allure")
        .x_title("time")
        .y_title("allure")
        .legend_title("Allure")
        .build()
        .write_html(cli.html_path);
    log::info!("plot done");
}
