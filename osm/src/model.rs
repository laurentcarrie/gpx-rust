pub struct Gpx {
    pub version: String,
    pub creator: String,
    pub metadata: Metadata,
    pub trk: Vec<Trkseg>,
}
pub struct Metadata {
    pub desc: String,
    pub time: String,
}

pub struct Trkseg {
    pub trkpt: Vec<Trkpt>,
}

pub struct Trkpt {
    pub lat: f32,
    pub lon: f32,
    pub time: String,
    pub hdop: f32,
}
