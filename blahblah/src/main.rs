mod fff;

use geo::prelude::*;
use geo::{coord,  Point};
use std::f64::consts::PI;
extern crate gpx;

use std::fs::File;
use std::io::{BufReader, BufWriter};

use gpx::{read, GpxVersion, Waypoint};
use gpx::{Gpx, Track, TrackSegment};
use crate::circle::ggg;

pub fn find_p(radius: f64, center: Point, theta: f64) -> Point {
    let mut p0 = center;
    let geo_coord = coord! {x:center.x() + radius * 2.0 * f64::cos(theta),y:center.y()+radius*2.0*f64::sin(theta)};
    let mut p1: Point = geo_coord.into();

    loop {
        let geo_coord2 = coord! {x:(p0.x()+p1.x())/2.0,y:(p0.y()+p1.y())/2.0};
        let p2: Point = geo_coord2.into();

        let distance = center.geodesic_distance(&p2);
        if distance < radius {
            p0 = p2
        } else {
            p1 = p2
        }
        let distance = p0.geodesic_distance(&p1);
        if distance < 0.01 { break };
    }
    return p0;
}


pub fn make_circle_track(name:String,radius:f64,center:Point) -> Track {
    let mut track_segment = TrackSegment { points: vec![] };
    for i in (0..360).step_by(1) {
        let theta = i as f64 / 360.0 * 2.0 * PI;
        let p = find_p(radius, center, theta);
        track_segment.points.push(Waypoint::new(p));
    }

    let track = Track {
        name: Some(format!("{} - {} m",name,radius)),
        comment: None,
        description: None,
        source: None,
        links: vec![],
        type_: None,
        number: None,
        segments: vec![track_segment],
    };
    return track ;

}

fn main() {
    // This XML file actually exists — try it for yourself!
    // let file = File::open("tests/fixtures/wikipedia_example.gpx").unwrap();
    let file = File::open("tests/fixtures/track_20240928_164244.gpx").unwrap();
    let reader = BufReader::new(file);

    // read takes any io::Read and gives a Result<Gpx, Error>.
    let gpx: Gpx = read(reader).unwrap();

    // Each GPX file has multiple "tracks", this takes the first one.
    // let track: &Track = &gpx.tracks[0];
    // assert_eq!(track.name, Some(String::from("Example GPX Document")));

    // Each track will have different segments full of waypoints, where a
    // waypoint contains info like latitude, longitude, and elevation.
    // let segment: &TrackSegment = &track.segments[0];

    // This is an example of retrieving the elevation (in meters) at certain points.
    // assert_eq!(segment.points[0].elevation, Some(4.46));
    // assert_eq!(segment.points[1].elevation, Some(4.94));
    // assert_eq!(segment.points[2].elevation, Some(6.87));

    // for track in &gpx.tracks {
    //     // println!("{}",track.name.as_ref().unwrap());
    //     for segment in &track.segments {
    //         for point in &segment.points {
    //             // dbg!(&point) ;
    //             // dbg!(&point.point()) ;
    //             // // println!("{} {}",point.name.as_ref().unwrap(),point.source.as_ref().unwrap()) ;
    //             // println!("{} {}",&point.point().x(),&point.point().y()) ;
    //         }
    //     }
    // }

    let p0 = gpx
        .tracks
        .first()
        .unwrap()
        .segments
        .first()
        .unwrap()
        .points
        .first()
        .unwrap()
        .point();
    // dbg!(&p0) ;
    let pn = gpx
        .tracks
        .last()
        .unwrap()
        .segments
        .last()
        .unwrap()
        .points
        .last()
        .unwrap()
        .point();
    // dbg!(&pn) ;

    // let distance = p0.geodesic_distance(&pn);
    // println!("distance : {}",distance) ;

    // let gare_de_saint_germain_en_laye:Point  = coord!{ x:2.094635928847278,y:48.89842169825594,}.into();

    let secretan :Point = coord!{x:2.377423253771183,y:48.87935687171258}.into() ;
    let malakoff:Point=coord!{x:2.2821949645035424,y:  48.81398973092065}.into() ;
    let tracks=vec![1000.0,2000.0,3000.0,4000.0,5000.0].iter().map(|r|  make_circle_track("".to_string(),*r ,malakoff)).collect() ;

    let  gpx = Gpx {
        version: GpxVersion::Gpx11,
        creator: None,
        metadata: None,
        waypoints: vec![],
        tracks: tracks,
        routes: vec![],
    };

    // Create file at path
    let gpx_file = File::create("circle-1000.gpx").unwrap();
    let buf = BufWriter::new(gpx_file);
    gpx::write(&gpx, buf).unwrap();


    // let mut reader = shapefile::Reader::from_path("tests/data/multipatch.shp")?;
    // for shape_record in reader.iter_shapes_and_records() {
    //     let (shape, record) = shape_record?;
    //     println!("{}", shape);
    // }

    ggg() ;

    println!("DONE");
}
