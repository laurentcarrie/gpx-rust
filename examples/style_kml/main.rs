use lc_gpx_utils::kml::model as M;
use lc_gpx_utils::kml::write::write_kml;
use lc_gpx_utils::tcx::read::df_of_tcx_file;
use lc_gpx_utils::transform::kml_of_df::kml_of_df;
use log;
use polars::prelude::Series;
use polars::prelude::*;
use std::path::PathBuf;

/// the most simple example,
pub fn main() {
    let _ = simple_logging::log_to_file("/tmp/foo.log", log::LevelFilter::Info);
    // read a garmin tcx file to a polars dataframe
    let df = df_of_tcx_file("tests/data/activity_19302390776.tcx".to_string()).expect("read");

    let _wanted_circuits = Series::new("wanted_circuits".into(), vec![3, 5]);

    let df = df
        .lazy()
        .with_column(
            when(col("circuit").eq(1))
                .then(lit("red-style"))
                .otherwise(lit("blue-style"))
                .alias("style"),
        )
        .with_column(
            col("time")
                .cast(DataType::Datetime(
                    TimeUnit::Milliseconds,
                    Some(TimeZone::UTC),
                ))
                .alias("t"),
        )
        .collect()
        .expect("add style column");

    // get a kml document from a polars dataframe

    let kml: M::Document = kml_of_df(df).expect("kml");

    // create two styles, green and red
    let red_style = M::Style {
        id: "red-style".to_string(),
        icon_style_scale: 1.0,
        icon_url: "https://earth.google.com/earth/document/icon?color=ff0000&id=2000&scale=2"
            .to_string(),
        line_style_width: 1.0,
    };
    let blue_style = M::Style {
        id: "blue-style".to_string(),
        icon_style_scale: 1.0,
        icon_url: "https://earth.google.com/earth/document/icon?color=0000ff&id=2000&cale=50"
            .to_string(),
        line_style_width: 1.0,
    };

    let kml = M::Document {
        styles: vec![red_style, blue_style],
        ..kml
    };

    // write the kml file to disk. Visualize it with google earth
    write_kml(&kml, &PathBuf::from("example-styled.kml")).expect("write");
}
