#[cfg(test)]
mod tests {
    use lc_gpx_utils::kml::model as M;
    use lc_gpx_utils::kml::write::write_kml;
    use lc_gpx_utils::tcx::read::df_of_tcx_file;
    use lc_gpx_utils::transform::kml_of_df::kml_of_df;
    // use polars::prelude::*;
    use std::path::PathBuf;

    #[test]
    fn test_kml_of_df() {
        let _ = simple_logging::log_to_file("/tmp/foo.log", log::LevelFilter::Info);
        let df = df_of_tcx_file("tests/data/activity_19302390776.tcx".to_string()).expect("read");
        log::info!("{:?}", df);
        let kml = kml_of_df(df).expect("kml");
        log::info!("{:?}", kml);
        assert_eq!(kml.elements.len(), 6);
        for e in kml.elements.iter() {
            match e {
                M::Element::EFolder(_) => continue,
                _ => panic!("not a folder"),
            }
        }
        write_kml(&kml, &PathBuf::from("test.kml")).expect("write");
    }
}
