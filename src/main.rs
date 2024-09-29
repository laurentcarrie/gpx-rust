use std::fs;
use serde::{Deserialize, Serialize};
// use serde_xml_rs::{from_str, to_string};
use serde_xml_rs::{from_str};


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Gpx {
    metadata: Metadata,
    trk: Vec<Event0>,
    // trkseg: Trkseg
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Metadata {
    desc: String,
    time: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Trk {
    #[serde(rename = "$value")]
    events: Vec<Event0>,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Trkseg {
    #[serde(rename = "$value")]
    events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum Event0 {
    Trkseg(Trkseg),
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum Event {
    Trkpt(Trkpt),
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Trkpt {
    lat: f32,
    lon: f32,
    time: String,
    hdop: f32,
}


fn main() {
    let data = fs::read_to_string("a.xml").unwrap();
    // println!("{}",&data) ;
    // let gpx: Gpx = from_str(&data).unwrap();
    let gpx: Gpx = from_str(&data).unwrap();
    // assert_eq!(plate_appearance.events[0], Event::Pitch(Pitch { speed: 95, r#type: PitchType::FourSeam, outcome: PitchOutcome::Ball }));
    // println!("{}",gpx.plate_appearance.events.len()) ;
    println!("desc: {}", gpx.metadata.desc);
    for trk in &gpx.trk {
        match trk {
            Event0::Trkseg(seg) => {
                for pt in &seg.events {
                    match pt {
                        Event::Trkpt(pt) => {
                            println!("lat:{}, lon:{}",pt.lat,pt.lon)
                        }
                    }
                }
                println ! ("xxxxx")
            }
        }
    }
    println!("DONE");
}

