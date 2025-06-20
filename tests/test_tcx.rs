#[cfg(test)]
mod tests {
    // use lc_gpx_utils::tcx::model as M;
    use log::LevelFilter;
    use polars::prelude::*;
    // use polars::prelude::* ;
    use plotlars::{Plot, Rgb, ScatterPlot};
    use std::collections::HashSet;
    use std::fs::File;
    #[test]
    fn test_tcx_1() {
        let _ = simple_logging::log_to_file("/tmp/foo.log", LevelFilter::Info)
            .expect("Failed to initialize logging");
        log::info!("start test 1");

        // let tcx_file = "tests/data/activity_19083574335.tcx";
        let tcx_file = "tests/data/activity_19302390776.tcx";
        let mut df =
            lc_gpx_utils::tcx::read::df_of_tcx_file(tcx_file.into()).expect("Failed get tcx");
        assert!(df.height() == 2951);

        let mut file: File = File::create("example.csv").expect("could not create file");

        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut df)
            .expect("write csv");
    }

    #[test]
    fn test_schema() {
        let _ = simple_logging::log_to_file("/tmp/foo.log", LevelFilter::Info)
            .expect("Failed to initialize logging");
        log::info!("start test schema");

        // let tcx_file = "tests/data/activity_19083574335.tcx";
        let tcx_file = "tests/data/activity_19302390776.tcx";
        let df = lc_gpx_utils::tcx::read::df_of_tcx_file(tcx_file.into()).expect("Failed get tcx");
        log::info!("computed schema : {:?}", df.schema());
        log::info!(
            "declared schema in tcx_df_schema : : {:?}",
            lc_gpx_utils::tcx::model::tcx_df_schema()
        );
        let h_expected: HashSet<(PlSmallStr, DataType)> =
            HashSet::from_iter(lc_gpx_utils::tcx::model::tcx_df_schema());
        let h_found: HashSet<(PlSmallStr, DataType)> = {
            let v = df
                .schema()
                .iter()
                .map(|(a, b)| ((*a).clone(), (*b).clone()))
                .collect();
            v
        };

        let h1 = h_expected.clone();
        let h2 = h_found.clone();

        let mut difference = h1.difference(&h2);
        if let Some(d) = &difference.next() {
            panic!(
                "expected field {:?} not found in found in computed output dataframe",
                d
            );
        }

        let h1 = h_expected.clone();
        let h2 = h_found.clone();
        let mut difference = h2.difference(&h1);
        if let Some(d) = &difference.next() {
            panic!(
                "found in computed dataframe field {:?} not found in expected dataframe",
                d
            );
        }
    }

    #[test]
    fn test_plot() {
        let _ = simple_logging::log_to_file("/tmp/foo.log", LevelFilter::Info)
            .expect("Failed to initialize logging");
        log::info!("start test plot");

        let dataset = LazyCsvReader::new("data/penguins.csv")
            .finish()
            .expect("read csv")
            .select([
                col("species").cast(DataType::Categorical(None, CategoricalOrdering::default())),
                col("flipper_length_mm").cast(DataType::Int16),
                col("body_mass_g").cast(DataType::Int16),
            ])
            .collect()
            .expect("x");

        let _ = ScatterPlot::builder()
            .data(&dataset)
            .x("body_mass_g")
            .y("flipper_length_mm")
            .group("species")
            .opacity(0.5)
            .size(12)
            .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
            .plot_title("Penguin Flipper Length vs Body Mass")
            .x_title("Body Mass (g)")
            .y_title("Flipper Length (mm)")
            .legend_title("Species")
            .build()
            .write_html("out.html");
        log::info!("plot done");
    }
}
