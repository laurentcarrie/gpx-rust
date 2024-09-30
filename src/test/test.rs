use std::fmt::Error;
use crate::iomodel::gpx_of_xml_string;
use crate::model::Gpx;
use std::{env, fs};
use std::path::Path;

#[test]
fn test_0() -> Result<(),Error> {
    let path = Path::new("./src/test/track_20240928_164244.gpx") ;
    let data = fs::read_to_string(path).unwrap();
    // println!("{}",&data) ;
    // let gpx: Gpx = from_str(&data).unwrap();
    let gpx: Gpx = gpx_of_xml_string(data).unwrap();
    // assert_eq!(plate_appearance.events[0], Event::Pitch(Pitch { speed: 95, r#type: PitchType::FourSeam, outcome: PitchOutcome::Ball }));
    // println!("{}",gpx.plate_appearance.events.len()) ;
    println!("desc : {}", gpx.metadata.desc);
    for seg in &gpx.trk {
        for pt in &seg.trkpt {
            println!("{} {}",pt.lat,pt.lon) ;
        }
    }
    println!("found {} segments",gpx.trk.len()) ;
    println!("DONE");
    return Ok(()) ;
}
