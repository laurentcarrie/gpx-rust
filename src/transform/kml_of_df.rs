use crate::kml::model as M;
use itertools::izip;
use log;
use polars::prelude::*;

fn circuits_of_df(df: &DataFrame) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let circuits = df
        .clone()
        .lazy()
        .select(vec![col("circuit")])
        .unique_stable(None, UniqueKeepStrategy::Any)
        .sort(vec!["circuit"], SortMultipleOptions::default())
        .collect()
        .expect("select");
    let objects = circuits.take_columns();
    let circuits = objects[0]
        .u32()?
        .iter()
        .map(|c| c.expect("circuit should not be null"))
        .collect::<Vec<_>>();
    Ok(circuits)
}

fn folder_of_circuit(
    df: &DataFrame,
    circuit: u32,
) -> Result<M::Element, Box<dyn std::error::Error>> {
    let df = df
        .clone()
        .lazy()
        .filter(col("circuit").eq(lit(circuit)))
        .unique_stable(None, UniqueKeepStrategy::Any)
        .sort(vec!["time"], SortMultipleOptions::default())
        .select(vec![col("x"), col("y")])
        .collect()
        .expect("select");
    let objects = df.take_columns();
    let iterx = objects[0].f64()?.iter();
    let itery = objects[1].f64()?.iter();

    let elements: Vec<M::Element> = izip!(iterx, itery)
        .into_iter()
        .map(|(x, y)| {
            M::Placemark::new_point_on_ground(
                "Notre Dame".to_string(),
                "home of Quasimodo".to_string(),
                Some("style-1".to_string()),
                x.expect("x"),
                y.expect("y"),
            )
            .into()
        })
        .collect();
    let f = M::Folder {
        name: format!("circuit {}", circuit),
        description: format!("desc {}", circuit),
        elements: elements,
    }
    .into();
    Ok(f)
}

pub fn kml_of_df(df: DataFrame) -> Result<M::Document, Box<dyn std::error::Error>> {
    let circuits = circuits_of_df(&df)?;
    log::info!("{:?}", &circuits);

    let folders: Result<Vec<M::Element>, _> = circuits
        .into_iter()
        .map(|c| folder_of_circuit(&df, c))
        .collect();
    // log::info!("{}:{} {:?}", file!(), line!(), elements);
    let kmldoc = M::Document {
        name: "a name".to_string(),
        description: "description".to_string(),
        styles: vec![],
        elements: folders?,
    };

    Ok(kmldoc)
}
