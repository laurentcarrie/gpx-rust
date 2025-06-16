use itertools::izip;
// use quote::{format_ident, quote};
// use core::panic;
// use geo::Point;
// use log;
// use regex;
// use std::io::BufWriter;
use tcx;

// use polars::prelude::Column;
use polars::prelude::Series;
use polars::prelude::*;
// use polars::prelude::{DataType, TimeUnit};

// use itertools::izip;

// use crate::tcx::model as M ;

// /// Parses a duration string in the format "MM:SS.mmm" into milliseconds.
// fn parse_duration(str_val: &Column) -> Column {
//     str_val
//         .str()
//         .expect("")
//         .into_iter()
//         .map(|opt_name: Option<&str>| {
//             opt_name.map(|name: &str| {
//                 match regex::Regex::new(
//                     r"(?P<minutes>\d+):(?P<seconds>\d+)\.?(?P<milliseconds>\d+)?",
//                 ) {
//                     Ok(re) => {
//                         if let Some(caps) = re.captures(name) {
//                             let minutes: u32 =
//                                 caps.name("minutes").unwrap().as_str().parse().unwrap();
//                             let seconds: u32 =
//                                 caps.name("seconds").unwrap().as_str().parse().unwrap();
//                             let milliseconds: u32 = match caps.name("milliseconds") {
//                                 Some(val) => val.as_str().parse().unwrap(),
//                                 None => 0,
//                             };
//                             minutes * 60000 + seconds * 1000 + milliseconds
//                         } else {
//                             log::warn!("Failed to parse duration: {}", name);
//                             panic!("Failed to parse duration: {}", name);
//                         }
//                     }
//                     Err(_) => {
//                         log::warn!("Invalid regex for duration parsing");
//                         panic!("Invalid regex for duration parsing");
//                     }
//                 }
//             })
//         })
//         .collect::<UInt32Chunked>()
//         .into_column()
// }

// /// Reads a TCX file and returns a DataFrame with the summary
// fn summary_training(tcx_file: &String) -> Result<DataFrame, Box<dyn std::error::Error>> {
//     let mut path = PathBuf::from(tcx_file);
//     path.set_extension("csv");
//     let file = File::open(path)?;
//     let df: DataFrame = CsvReader::new(file).finish()?;
//     let mut df = df
//         .lazy()
//         .select(vec![
//             col("Intervalle"),
//             col("Type d'étape"),
//             col("Circuit").alias("Circuit_str"),
//             col("Allure moyenne"),
//             col("Durée").alias("Duree_str"),
//             col("Durée").alias("Duree_u32"),
//             col("Distance"),
//             col("Temps cumulé").alias("Temps_cumule_str"),
//             col("Temps cumulé").alias("Temps_cumule_u32"),
//         ])
//         .collect()?;

//     // log::info!("{}:{}", file!(), line!());
//     df.apply("Duree_u32", parse_duration)?;
//     // log::info!("{}:{}", file!(), line!());
//     df.apply("Temps_cumule_u32", parse_duration)?;
//     let df = df
//         .lazy()
//         .with_columns(vec![
//             col("Duree_u32")
//                 .cast(DataType::Duration(TimeUnit::Milliseconds))
//                 .alias("Durée"),
//         ])
//         .with_columns(vec![
//             col("Temps_cumule_u32")
//                 .cast(DataType::Duration(TimeUnit::Milliseconds))
//                 .alias("Temps cumulé"),
//         ])
//         .with_columns(vec![
//             col("Circuit_str").cast(DataType::UInt32).alias("Circuit"),
//         ])
//         .collect()?;

//     let df = df
//         .lazy()
//         .select(vec![
//             col("Intervalle"),
//             col("Type d'étape"),
//             col("Circuit"),
//             col("Allure moyenne"),
//             col("Durée"),
//             col("Temps cumulé"),
//             col("Distance"),
//         ])
//         .collect()?;

//     // log::info!("Summary DataFrame: {:?}", df);

//     Ok(df)
// }

// fn summary_race(tcx_file: &String) -> Result<DataFrame, Box<dyn std::error::Error>> {
//     let mut path = PathBuf::from(tcx_file);
//     path.set_extension("csv");
//     let file = File::open(path)?;
//     let df: DataFrame = CsvReader::new(file).finish()?;
//     let df = df
//         .lazy()
//         .with_columns(vec![
//             col("Circuits").cast(DataType::UInt32).alias("Circuit"),
//         ])
//         .with_columns(vec![lit("no type").alias("Type d'étape")])
//         .collect()?;

//     log::info!("¨{}:{} Summary DataFrame: {:?}", file!(), line!(), df);

//     Ok(df)
// }

// /// the csv file associated to a garmin tcx does not always have the same format
// /// for now, we found either race or training
// fn _summary(tcx_file: &String) -> Result<DataFrame, Box<dyn std::error::Error>> {
//     let ret1 = summary_training(tcx_file);
//     if ret1.is_ok() {
//         return ret1;
//     }
//     let ret2 = summary_race(tcx_file);
//     if ret2.is_ok() {
//         return ret2;
//     }
//     log::error!("{:?}", ret1);
//     log::error!("{:?}", ret2);
//     Err(String::from("training and race failed").into())
// }

/// reads a tcx garmin file, and the associated csv file (same name, with .csv extension)
/// that must be in  the same directory. This is provided by garmin-connect export, on the web site
/// <https://connect.garmin.com>
///
/// when doing a garmin training, each interval of the training is stored as a tcx activity.
///
/// this will create a polars dataframe from the garmin file tcx file. The tcx activity is transformed
/// to a `circuit` field index, starting with 1.
///
/// the csv file contains a summary of the training, we are interhat
///
/// and join the two on the circuit field so you can have the interval description
pub fn get_df(tcx_file: String) -> Result<DataFrame, Box<dyn std::error::Error>> {
    // let summary: DataFrame = summary(&tcx_file)?;

    let tcx = tcx::read_file(tcx_file.as_str())?;
    let activities = tcx.activities.as_ref().expect("activities");

    // for a garmin workout, there is only one activity
    assert!(
        activities.activities.len() == 1,
        "Expected exactly one activity in the TCX file"
    );
    let activity = activities.activities.first().expect("first activity");
    let mut v: Vec<(f64, f64, f64, f64, i64, u32)> = vec![];

    // the laps of the activity are the steps of the workout
    for (i, lap) in activity.laps.iter().enumerate() {
        for track in lap.tracks.iter() {
            for p in track.trackpoints.iter() {
                let t = p.time;
                // let t = chrono::DateTime::parse_from_rfc3339(&s)?;
                if let Some(position) = &p.position {
                    v.push((
                        position.longitude,
                        position.latitude,
                        p.distance_meters.unwrap_or(0.0),
                        p.heart_rate.clone().map(|p| p.value).unwrap_or(0.0),
                        // p.extensions
                        //     .clone()
                        //     .map(|e| e.tpx)
                        //     .flatten()
                        //     .map(|e| e.speed)
                        //     .flatten(),
                        t.timestamp_millis(),
                        i as u32 + 1,
                    ));
                };
            }
        }
    }
    let x = Series::new("x".into(), v.iter().map(|x| x.0).collect::<Vec<_>>());
    let y = Series::new("y".into(), v.iter().map(|x| x.1).collect::<Vec<_>>());
    let distance = Series::new("distance".into(), v.iter().map(|x| x.2).collect::<Vec<_>>());
    let hr = Series::new("hr".into(), v.iter().map(|x| x.3).collect::<Vec<_>>());
    // let time = Series::new("time".into(), v.iter().map(|x| (*x).4).collect::<Vec<_>>());
    let circuit = Series::new("circuit".into(), v.iter().map(|x| x.5).collect::<Vec<_>>());

    let (_speed0, _allure0, speed, allure) = {
        let iter_time_a = v[0..v.len() - 2].iter().map(|x| x.4);
        let iter_time_b = v[1..v.len() - 1].iter().map(|x| x.4);
        let iter_time_c = v[2..v.len()].iter().map(|x| x.4);
        let iter_distance_a = v[0..v.len() - 2].iter().map(|x| x.2);
        let iter_distance_b = v[1..v.len() - 1].iter().map(|x| x.2);
        let iter_distance_c = v[2..v.len()].iter().map(|x| x.2);

        let mut speed0: Vec<f64> = vec![];
        let mut allure0 = vec![];
        let mut speed: Vec<f64> = vec![];
        let mut allure = vec![];

        // dx in meters, dt in ms, we want speed in km/h and allure in mn/km
        let kv = 3600.0;
        let ka = 1.0 / 60.0;
        for (d0, d1, d2, t0, t1, t2) in izip!(
            iter_distance_a,
            iter_distance_b,
            iter_distance_c,
            iter_time_a,
            iter_time_b,
            iter_time_c
        ) {
            let v = (d1 - d0) / ((t1 - t0) as f64);
            speed0.push(kv * v);
            allure0.push(ka / v);

            let v = (-3.0 * d0 + 4.0 * d1 - d2) / ((t2 - t0) as f64);
            speed.push(kv * v);
            allure.push(ka / v);
        }
        speed0.insert(0, *speed0.first().expect("not empty"));
        speed0.insert(0, *speed0.first().expect("not empty"));
        assert_eq!(v.len(), speed0.len());
        allure0.insert(0, *allure0.first().expect("not empty"));
        allure0.insert(0, *allure0.first().expect("not empty"));
        assert_eq!(v.len(), allure0.len());
        speed.insert(0, *speed.first().expect("not empty"));
        speed.insert(0, *speed.first().expect("not empty"));
        assert_eq!(v.len(), speed.len());
        allure.insert(0, *allure.first().expect("not empty"));
        allure.insert(0, *allure.first().expect("not empty"));
        assert_eq!(v.len(), allure.len());
        (
            Series::new("speed0".into(), speed0),
            Series::new("allure0".into(), allure0),
            Series::new("speed".into(), speed),
            Series::new("allure".into(), allure),
        )
    };
    let df = DataFrame::new(vec![
        x.into(),
        y.into(),
        hr.into(),
        // time.into(),
        distance.into(),
        circuit.into(),
        // speed0.into(),
        // allure0.into(),
        speed.into(),
        allure.into(),
    ])
    .unwrap();

    // let df: DataFrame = df
    //     .lazy()
    //     .with_columns(vec![
    //         col("time")
    //             .cast(DataType::Datetime(
    //                 TimeUnit::Milliseconds,
    //                 Some(TimeZone::UTC),
    //             ))
    //             .alias("t"),
    //     ])
    //     .collect()?;

    // let df = df
    //     .lazy()
    //     .join(
    //         summary.clone().lazy(),
    //         [col("circuit")],
    //         [col("Circuit")],
    //         JoinArgs::default(),
    //     )
    //     .collect()?;

    Ok(df)
}
