use quote::{format_ident, quote};
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

pub fn points_of_multipolygon(p:&Vec<PolygonType>) -> Vec<Vec<TrackSegment>> {
    let mut ret : Vec<Vec<TrackSegment>>= vec![] ;
    for pp in p {
        let v_tracksegment = points_of_polygon(pp) ;
            ret.push(v_tracksegment) ;
    }
    return ret ;
}


pub fn make_tracks(f: &Feature) -> (String,String,Vec<Track>) {
    let p = f.properties.clone();
    let mut nom: String = "".to_string();
    let mut code: String = "".to_string();
    for (k, v) in p.unwrap() {
        if k == "nom".to_string() {
            nom = v.to_string().replace("\"","");
        } else if k == "code".to_string() {
            code = v.to_string().replace("\"","");
        }
    }
    println!("{} {}",code,nom) ;
    let geometry: &Geometry = &f.geometry.clone().unwrap();
    let v_track_segments  = match &geometry.value
    {
        Value::Polygon(p) => { vec![points_of_polygon(p)] }
        Value::MultiPolygon(p) => { points_of_multipolygon(p) }
        _ => { dbg!( &geometry.value) ; panic!("bad type")}
    } ;
    println!("nb segments : {}",v_track_segments.len()) ;

    let mut tracks=vec![] ;

    for v_track_segment in v_track_segments {
        for s in &v_track_segment {
            println!(" nb points : {}", s.points.len());
        }


        let track = Track {
            name: Some(format!("{} - {}", code, nom)),
            comment: None,
            description: None,
            source: None,
            links: vec![],
            type_: None,
            number: None,
            segments: v_track_segment,
        };
        tracks.push(track);
    }

    return (nom,code,tracks);
}

pub fn ggg() {
    let geojson_str: String = fs::read_to_string("contour-des-departements.geojson").unwrap();
    let geojson: GeoJson = geojson_str.parse::<GeoJson>().unwrap();
    let fc: FeatureCollection = FeatureCollection::try_from(geojson).unwrap();
    for f in fc.features.iter() {
        let (name,code,tracks) = make_tracks(f);
        let gpx = Gpx {
            version: GpxVersion::Gpx11,
            creator: None,
            metadata: None,
            waypoints: vec![],
            tracks: tracks,
            routes: vec![],
        };

        // Create file at path
        let path = format!("departements/{code}.gpx",code=code) ;
        let gpx_file = File::create(path).unwrap();
        let buf = BufWriter::new(gpx_file);
        gpx::write(&gpx, buf).unwrap();
    }

    // println!("code : {}",p.get(s)) ;
}
