pub mod iomodel;
pub mod model;
pub mod myerror;

#[cfg(test)]
pub mod test;


// fn main() {
//     let data = fs::read_to_string("a.xml").unwrap();
//     // println!("{}",&data) ;
//     // let gpx: Gpx = from_str(&data).unwrap();
//     let gpx: Gpx = gpx_of_xml_string(data).unwrap();
//     // assert_eq!(plate_appearance.events[0], Event::Pitch(Pitch { speed: 95, r#type: PitchType::FourSeam, outcome: PitchOutcome::Ball }));
//     // println!("{}",gpx.plate_appearance.events.len()) ;
//     println!("desc : {}", gpx.metadata.desc);
//     for seg in &gpx.trk {
//         for pt in &seg.trkpt {
//             println!("{} {}",pt.lat,pt.lon) ;
//         }
//     }
//     println!("found {} segments",gpx.trk.len()) ;
//     println!("DONE");
// }

extern crate gpx;

use std::io::BufReader;
use std::fs::File;

use gpx::read;
use gpx::{Gpx, Track, TrackSegment};

fn main() {
    // This XML file actually exists — try it for yourself!
    let file = File::open("tests/track.gpx").unwrap();
    let reader = BufReader::new(file);

    // read takes any io::Read and gives a Result<Gpx, Error>.
    let gpx: Gpx = read(reader).unwrap();

    // Each GPX file has multiple "tracks", this takes the first one.
    let track: &Track = &gpx.tracks[0];
    assert_eq!(track.name, Some(String::from("Example GPX Document")));

    // Each track will have different segments full of waypoints, where a
    // waypoint contains info like latitude, longitude, and elevation.
    let segment: &TrackSegment = &track.segments[0];

    // This is an example of retrieving the elevation (in meters) at certain points.
    assert_eq!(segment.points[0].elevation, Some(4.46));
    assert_eq!(segment.points[1].elevation, Some(4.94));
    assert_eq!(segment.points[2].elevation, Some(6.87));
}