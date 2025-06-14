use std::path::PathBuf;
// use quote::{format_ident, quote};
use core::panic;
use geo::Point;
use gpx::{Gpx, Track, TrackSegment};
use gpx::{GpxVersion, Waypoint};
use log;
use regex;
use std::fs::File;
use std::io::BufWriter;
use tcx;

use polars::prelude::Column;
use polars::prelude::Series;
use polars::prelude::*;
use polars::prelude::{DataType, TimeUnit, TimeZone};

use itertools::izip;

pub fn make_tracks(df: &DataFrame, circuits: Vec<(u32, String)>) -> Vec<Track> {
    let mut tracks: Vec<Track> = Vec::new();
    for (circuit, name) in circuits {
        let mut trkseg: TrackSegment = TrackSegment::new();
        let df = df
            .clone()
            .lazy()
            .filter(col("circuit").eq(lit(circuit)))
            .select(vec![col("x"), col("y")])
            .collect()
            .expect("Failed to collect filtered DataFrame");

        let objects = df.take_columns();
        let iterx = objects[0].f64().expect("col 0").iter();
        let itery = objects[1].f64().expect("col 1").iter();

        for (x, y) in izip!(iterx, itery) {
            let point = Waypoint::new(Point::new(x.expect("x"), y.expect("y")));
            trkseg.points.push(point);
        }

        let trksegs: Vec<TrackSegment> = vec![trkseg];

        let track = Track {
            name: Some(format!("circuit {} - {}", circuit, name)),
            comment: None,
            description: None,
            source: None,
            links: vec![],
            type_: None,
            number: None,
            segments: trksegs,
        };

        tracks.push(track);
    }

    tracks
}

/// Parses a duration string in the format "MM:SS.mmm" into milliseconds.
fn parse_duration(str_val: &Column) -> Column {
    str_val
        .str()
        .expect("")
        .into_iter()
        .map(|opt_name: Option<&str>| {
            opt_name.map(|name: &str| {
                match regex::Regex::new(
                    r"(?P<minutes>\d+):(?P<seconds>\d+)\.?(?P<milliseconds>\d+)?",
                ) {
                    Ok(re) => {
                        if let Some(caps) = re.captures(name) {
                            let minutes: u32 =
                                caps.name("minutes").unwrap().as_str().parse().unwrap();
                            let seconds: u32 =
                                caps.name("seconds").unwrap().as_str().parse().unwrap();
                            let milliseconds: u32 = match caps.name("milliseconds") {
                                Some(val) => val.as_str().parse().unwrap(),
                                None => 0,
                            };
                            minutes * 60000 + seconds * 1000 + milliseconds
                        } else {
                            log::warn!("Failed to parse duration: {}", name);
                            panic!("Failed to parse duration: {}", name);
                        }
                    }
                    Err(_) => {
                        log::warn!("Invalid regex for duration parsing");
                        panic!("Invalid regex for duration parsing");
                    }
                }
            })
        })
        .collect::<UInt32Chunked>()
        .into_column()
}

/// Reads a TCX file and returns a DataFrame with the summary
pub fn summary_training(tcx_file: &String) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(tcx_file);
    path.set_extension("csv");
    let file = File::open(path)?;
    let df: DataFrame = CsvReader::new(file).finish()?;
    log::info!("Columns: {:?}", df.get_column_names());
    log::info!("Shape: {:?}", df.shape());
    log::info!("Summary DataFrame: {:?}", df);
    log::info!("{}:{}", file!(), line!());
    let mut df = df
        .lazy()
        .select(vec![
            col("Intervalle"),
            col("Type d'étape"),
            col("Circuit").alias("Circuit_str"),
            col("Allure moyenne"),
            col("Durée").alias("Duree_str"),
            col("Durée").alias("Duree_u32"),
            col("Distance"),
            col("Temps cumulé").alias("Temps_cumule_str"),
            col("Temps cumulé").alias("Temps_cumule_u32"),
        ])
        .collect()?;

    log::info!("{}:{}", file!(), line!());
    df.apply("Duree_u32", parse_duration)?;
    log::info!("{}:{}", file!(), line!());
    df.apply("Temps_cumule_u32", parse_duration)?;
    let df = df
        .lazy()
        .with_columns(vec![
            col("Duree_u32")
                .cast(DataType::Duration(TimeUnit::Milliseconds))
                .alias("Durée"),
        ])
        .with_columns(vec![
            col("Temps_cumule_u32")
                .cast(DataType::Duration(TimeUnit::Milliseconds))
                .alias("Temps cumulé"),
        ])
        .with_columns(vec![
            col("Circuit_str").cast(DataType::UInt32).alias("Circuit"),
        ])
        .collect()?;

    let df = df
        .lazy()
        .select(vec![
            col("Intervalle"),
            col("Type d'étape"),
            col("Circuit"),
            col("Allure moyenne"),
            col("Durée"),
            col("Temps cumulé"),
            col("Distance"),
        ])
        .collect()?;

    log::info!("Summary DataFrame: {:?}", df);

    Ok(df)
}

pub fn summary_race(tcx_file: &String) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(tcx_file);
    path.set_extension("csv");
    let file = File::open(path)?;
    let df: DataFrame = CsvReader::new(file).finish()?;
    log::info!("Columns: {:?}", df.get_column_names());
    log::info!("Shape: {:?}", df.shape());
    log::info!("Summary DataFrame: {:?}", df);
    log::info!("{}:{}", file!(), line!());
    let df = df
        .lazy()
        .with_columns(vec![
            col("Circuits").cast(DataType::UInt32).alias("Circuit"),
        ])
        .with_columns(vec![lit("no type").alias("Type d'étape")])
        .collect()?;

    log::info!("¨{}:{} Summary DataFrame: {:?}", file!(), line!(), df);

    Ok(df)
}

pub fn summary(tcx_file: &String) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let ret1 = summary_training(tcx_file);
    if ret1.is_ok() {
        return ret1;
    }
    let ret2 = summary_race(tcx_file);
    if ret2.is_ok() {
        return ret2;
    }
    log::error!("{:?}", ret1);
    log::error!("{:?}", ret2);
    Err(String::from("training and race failed").into())
}

/// write the course data to a gpx file
pub fn get_gpx(tcx_file: String) -> Result<Gpx, Box<dyn std::error::Error>> {
    let summary: DataFrame = summary(&tcx_file)?;

    let tcx = tcx::read_file(tcx_file.as_str())?;
    let activities = tcx.activities.as_ref().expect("activities");
    assert!(
        activities.activities.len() == 1,
        "Expected exactly one activity in the TCX file"
    );
    let activity = activities.activities.first().expect("first activity");
    let mut v: Vec<(f64, f64, i64, u32)> = vec![];

    for (i, lap) in activity.laps.iter().enumerate() {
        for track in lap.tracks.iter() {
            for p in track.trackpoints.iter() {
                let t = p.time;
                // let t = chrono::DateTime::parse_from_rfc3339(&s)?;
                if let Some(position) = &p.position {
                    v.push((
                        position.longitude,
                        position.latitude,
                        t.timestamp_millis(),
                        i as u32 + 1,
                    ));
                };
            }
        }
    }
    let x = Series::new(
        "x".into(),
        v.iter().map(|(x, _, _, _)| *x).collect::<Vec<_>>(),
    );
    let y = Series::new(
        "y".into(),
        v.iter().map(|(_, y, _, _)| *y).collect::<Vec<_>>(),
    );
    let time = Series::new(
        "time".into(),
        v.iter().map(|(_, _, t, _)| *t).collect::<Vec<_>>(),
    );
    let circuit = Series::new(
        "circuit".into(),
        v.iter().map(|(_, _, _, c)| *c).collect::<Vec<_>>(),
    );

    let df = DataFrame::new(vec![x.into(), y.into(), time.into(), circuit.into()]).unwrap();

    let df: DataFrame = df
        .lazy()
        .with_columns(vec![
            col("time")
                .cast(DataType::Datetime(
                    TimeUnit::Milliseconds,
                    Some(TimeZone::UTC),
                ))
                .alias("t"),
        ])
        .collect()?;

    log::info!("DataFrame with time: {:?}", df);

    let df = df
        .lazy()
        .join(
            summary.clone().lazy(),
            [col("circuit")],
            [col("Circuit")],
            JoinArgs::default(),
        )
        .collect()?;

    log::info!("Joined DataFrame: {:?}", df);

    let infos: Vec<(u32, String)> = {
        let df = &summary.clone();
        let df = df
            .clone()
            .lazy()
            .select(vec![col("Circuit"), col("Type d'étape")])
            .collect()?;
        let objects = df.take_columns();
        let iterc = objects[0].u32().unwrap().iter();
        let itert = objects[1].str().unwrap().iter();
        let mut ret: Vec<(u32, String)> = vec![];
        for (c, t) in izip!(iterc, itert) {
            match (c, t) {
                (Some(c), Some(t)) => {
                    ret.push((c, t.to_string()));
                }
                (None, Some(t)) => {
                    log::warn!("Circuit is None, type: {}", t);
                    continue;
                }
                (Some(c), None) => {
                    log::warn!("Type is None, circuit: {}", c);
                    continue;
                }
                (None, None) => {
                    log::warn!("Both circuit and type are None");
                    continue;
                }
            }
        }
        ret
    };

    let tracks = make_tracks(&df, infos);

    let gpx = Gpx {
        version: GpxVersion::Gpx11,
        creator: None,
        metadata: None,
        waypoints: vec![],
        tracks,
        routes: vec![],
    };

    // let gpx_file = File::create(gpx_file).unwrap();
    // let buf = BufWriter::new(gpx_file);
    // gpx::write(&gpx, buf).unwrap();

    Ok(gpx)
}

pub fn write_gpx(gpx: &Gpx, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let gpx_file = File::create(path)?;
    let buf = BufWriter::new(gpx_file);
    gpx::write(gpx, buf).unwrap();
    Ok(())
}

/// blah blah
/// # Examples:
/// ```
/// use lc_gpx_utils::course::xcourse;
/// let a = xcourse::add(1, 2);
/// assert_eq!(a, 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
