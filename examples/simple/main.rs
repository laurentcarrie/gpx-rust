use lc_gpx_utils::kml::model as M;
use lc_gpx_utils::kml::write::write_kml;
use lc_gpx_utils::tcx::read::df_of_tcx_file;
use lc_gpx_utils::transform::kml_of_df::kml_of_df;
use polars::prelude::*;
use std::path::PathBuf;

/// the most simple example,
pub fn main() {
    // read a garmin tcx file to a polars dataframe
    let df: DataFrame =
        df_of_tcx_file("tests/data/activity_19302390776.tcx".to_string()).expect("read");
    // get a kml document from a polars dataframe
    let kml: M::Document = kml_of_df(df).expect("kml");
    // write the kml file to disk. Visualize it with google earth
    write_kml(&kml, &PathBuf::from("test.kml")).expect("write");
}
