use crate::iomodel::gpx_of_xml_string;
use crate::model::Gpx;
use std::fmt::Error;
use std::path::Path;
use std::{env, fs};

#[test]
fn test_0() -> Result<(), Error> {
    let path = Path::new("./src/test/track_20240928_164244.gpx");
    let data = fs::read_to_string(path).unwrap();
    let gpx: Gpx = gpx_of_xml_string(data).unwrap();
    println!("desc : {}", gpx.metadata.desc);
    for seg in &gpx.trk {
        for pt in &seg.trkpt {
            println!("{} {}", pt.lat, pt.lon);
        }
    }
    println!("found {} segments", gpx.trk.len());
    println!("DONE");
    return Ok(());
}
