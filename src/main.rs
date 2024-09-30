pub mod iomodel;
pub mod model;
pub mod myerror;

#[cfg(test)]
pub mod test;

use crate::iomodel::gpx_of_xml_string;
use crate::model::Gpx;
use std::fs;

fn main() {
    println!("DONE") ;
}

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
