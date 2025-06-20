use crate::kml::model as M;
use chrono;
use itertools::izip;
use log;
use polars::prelude::*;

fn placemark_of_position(
    circuit: u32,
    x: Option<f64>,
    y: Option<f64>,
    style: Option<&str>,
    t: Option<i64>,
    allure: Option<f64>,
) -> Result<M::Element, Box<dyn std::error::Error>> {
    let t =
        chrono::DateTime::<chrono::Utc>::from_timestamp_millis(t.ok_or("t")?).ok_or("timestamp")?;
    let tstr = t.format("%+");
    let p = M::Placemark::new_point_on_ground(
        format!("{}", tstr).to_string(),
        format!(
            "<ul><li>circuit : {}</li><li>time : {}</li><li>allure : {}</li></ul>",
            circuit,
            tstr,
            allure.ok_or("allure")?
        ),
        style.map(|s| s.to_string()),
        x.ok_or("x")?,
        y.ok_or("y")?,
    )
    .into();
    Ok(p)
}

pub fn circuits_of_df(df: &DataFrame) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
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
    // if there is a style column, use it, else create a column.
    // we should create a column of Null, but don't know how to do it
    // kml reader will silently ignore non-existing style
    let df = if df.schema().get("style").is_some() {
        df.clone().lazy()
    } else {
        df.clone()
            .lazy()
            .with_columns([lit("no-style-found ").alias("style")])
    };

    let df = df
        .clone()
        .lazy()
        .filter(col("circuit").eq(lit(circuit)))
        .sort(vec!["time"], SortMultipleOptions::default())
        // .limit(10)
        .select(vec![
            col("x"),
            col("y"),
            col("style"),
            col("time"),
            col("allure"),
        ])
        .collect()
        .expect("select");

    log::info!("{}:{} {:?}", file!(), line!(), &df);

    let objects = df.take_columns();
    let iterx = objects[0].f64()?.iter();
    let itery = objects[1].f64()?.iter();
    let iterstyle = objects[2].str()?.iter();
    let itertime = objects[3].i64()?.iter();
    let iterallure = objects[4].f64()?.iter();

    let elements: Result<Vec<M::Element>, _> = izip!(iterx, itery, iterstyle, itertime, iterallure)
        .map(|(x, y, style, t, allure)| placemark_of_position(circuit, x, y, style, t, allure))
        .collect();
    let f = M::Folder {
        name: format!("circuit {}", circuit),
        description: format!("desc {}", circuit),
        elements: elements?,
    }
    .into();
    Ok(f)
}

/// blah blah blah
pub fn kml_of_df(df: DataFrame) -> Result<M::Document, Box<dyn std::error::Error>> {
    let circuits = circuits_of_df(&df)?;
    log::info!("{:?}", &circuits);

    let folders: Result<Vec<M::Element>, _> = circuits
        .into_iter()
        .map(|c| folder_of_circuit(&df, c))
        .collect();
    let folder = M::Folder {
        name: "root".to_string(),
        description: "".to_string(),
        elements: folders?,
    };
    // log::info!("{}:{} {:?}", file!(), line!(), elements);
    let kmldoc = M::Document {
        name: "a name".to_string(),
        description: "description".to_string(),
        styles: vec![],
        elements: vec![folder.into()],
    };

    Ok(kmldoc)
}
