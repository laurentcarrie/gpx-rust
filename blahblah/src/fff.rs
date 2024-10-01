use std::convert::TryFrom;
use std::fs;
use std::fs::File;
use std::io::BufWriter;

use geo::{coord, Point};
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, PolygonType, Value};
use gpx::{Gpx, GpxVersion, Track, TrackSegment, Waypoint};

pub fn points_of_polygon(p:&PolygonType) -> Vec<TrackSegment> {
    let mut v_tracksegment : Vec<TrackSegment> = vec![] ;

    for x in p.iter() {
        let mut points : Vec<Waypoint> = vec![] ;
        for xx in x.iter() {
            let xxx = xx.get(0).unwrap().clone() ;
            let yyy = xx.get(1).unwrap().clone() ;
            let geo_coord = coord! {x:xxx,y:yyy} ;
             let p: Point = geo_coord.into();
             points.push(Waypoint::new(p)) ;
        }
        let track_segment = TrackSegment { points: points };
        v_tracksegment.push(track_segment)
    }
    return v_tracksegment ;
}

pub fn points_of_multypolygon(p:&Vec<PolygonType>) -> Vec<TrackSegment> {
    let ret : Vec<TrackSegment>= vec![] ;
    return ret ;
}


pub fn make_track(f: &Feature) -> Track{
    let p = f.properties.clone();
    let mut nom: String = "".to_string();
    let mut code: String = "".to_string();
    for (k, v) in p.unwrap() {
        if k == "nom".to_string() {
            nom = v.to_string();
        } else if k == "code".to_string() {
            code = v.to_string();
        }
    }
    println!("{} {}",code,nom) ;
    let geometry: &Geometry = &f.geometry.clone().unwrap();
    let v_track_segment  = match &geometry.value
    {
        Value::Polygon(p) => { points_of_polygon(p) }
        Value::MultiPolygon(p) => { points_of_multypolygon(p) }
        _ => { dbg!( &geometry.value) ; panic!("bad type")}
    } ;
    println!("nb segments : {}",v_track_segment.len()) ;
    for s in &v_track_segment {
        println!(" nb points : {}", s.points.len()) ;
    }


    let track = Track {
        name: Some(format!("{} - {} m", code, nom)),
        comment: None,
        description: None,
        source: None,
        links: vec![],
        type_: None,
        number: None,
        segments: v_track_segment,
    };

    return track;
}

pub fn ggg() {
    let geojson_str: String = fs::read_to_string("contour-des-departements.geojson").unwrap();
    let geojson: GeoJson = geojson_str.parse::<GeoJson>().unwrap();
    let fc: FeatureCollection = FeatureCollection::try_from(geojson).unwrap();
    let tracks:Vec<Track>= fc.features.iter().map(|f| make_track(f)).collect();
    let gpx = Gpx {
        version: GpxVersion::Gpx11,
        creator: None,
        metadata: None,
        waypoints: vec![],
        tracks: tracks[0..10].to_vec(),
        routes: vec![],
    };

    // Create file at path
    let gpx_file = File::create("deps.gpx").unwrap();
    let buf = BufWriter::new(gpx_file);
    gpx::write(&gpx, buf).unwrap();

    // println!("code : {}",p.get(s)) ;
}
