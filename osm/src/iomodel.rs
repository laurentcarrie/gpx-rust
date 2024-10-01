use crate::model::{Gpx, Metadata, Trkpt, Trkseg};
use crate::myerror::MyError;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IOGpx {
    version: String,
    creator: String,
    metadata: IOMetadata,
    trk: Vec<Event0>,
    // trkseg: Trkseg
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IOMetadata {
    desc: String,
    time: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IOTrk {
    #[serde(rename = "$value")]
    events: Vec<Event0>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IOTrkseg {
    #[serde(rename = "$value")]
    events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum Event0 {
    Trkseg(IOTrkseg),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Event {
    Trkpt(IOTrkpt),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IOTrkpt {
    lat: f32,
    lon: f32,
    time: String,
    hdop: f32,
}

fn seg_of_ioseg(input: &Event0) -> Trkseg {
    let events = match input {
        Event0::Trkseg(iotrkseg) => &iotrkseg.events,
    };
    let vpoints: Vec<&IOTrkpt> = events
        .iter()
        .map(|e| match e {
            Event::Trkpt(p) => p,
        })
        .collect();

    let points: Vec<Trkpt> = vpoints
        .iter()
        .map(|p| Trkpt {
            lat: p.lat.clone(),
            lon: p.lon.clone(),
            time: p.time.clone(),
            hdop: p.hdop.clone(),
        })
        .collect();
    let seg = Trkseg { trkpt: points };
    return seg;
}

pub fn gpx_of_xml_string(data: String) -> Result<Gpx, MyError> {
    let iogpx: IOGpx = from_str(&data)?;

    let metadata = Metadata {
        time: iogpx.metadata.time,
        desc: iogpx.metadata.desc,
    };
    let segs = iogpx.trk.iter().map(|item| seg_of_ioseg(&item)).collect();
    // let segs = vec![] ;

    let gpx: Gpx = Gpx {
        version: iogpx.version,
        creator: iogpx.creator,
        metadata: metadata,
        trk: segs,
    };

    return Ok(gpx);
}
