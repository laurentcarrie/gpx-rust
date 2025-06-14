#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    /// the id, used as the `style_id` field in Placemark  
    pub id: String,
    pub icon_style_scale: f64,
    /// look at <https://kml4earth.appspot.com/icons.html>for instance
    pub icon_url: String,
    pub line_style_width: f64,
}

/// a Point
/// to have the point on the earth ground, set altitude to 0.0 and altitude_mode to [AltitudeMode::ErelativeToGround]
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub longitude: f64,
    pub latitude: f64,
    pub altitude: f64,
    pub altitude_mode: AltitudeMode,
}

/// a kml line string.
#[derive(Debug, Clone, PartialEq)]
pub struct LineString {
    pub extrude: u32,
    pub tessellate: u32,
    /// the altitude_mode of the points is ignored, altitude_mode of self is used instead
    pub altitude_mode: AltitudeMode,
    pub points: Vec<Point>,
}

/// @todo kml has other modes, for diving under sea, not implemented yet
#[derive(Debug, Clone, PartialEq)]
pub enum AltitudeMode {
    ErelativeToGround,
    /// 0 is the sea level
    Eabsolute,
}

/// a kml place marker, This is what you see in google earth, in the left tab
#[derive(Debug, Clone, PartialEq)]
pub struct Placemark {
    pub name: String,
    /// description. @todo : support CDATA, so you can have html code in the description
    pub description: String,
    /// a reference to the style, without the leading # as in the generated kml file
    pub style_id: Option<String>,
    /// a place marker has only one geo_element
    pub geo_element: GeoElement,
    pub visible: bool,
}

/// a kml folder, only contains placemarks or sub-folders
#[derive(Debug, Clone, PartialEq)]
pub struct Folder {
    pub name: String,
    pub description: String,
    pub elements: Vec<Element>,
}

/// a geoelement is the sum type for what is in a placemark
#[derive(Debug, Clone, PartialEq)]
pub enum GeoElement {
    EPoint(Point),
    ELineString(LineString),
}

/// a kml element, in this simple model, is either a folder or a placemark,
/// as you see in google earth or map in the left tab
#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    EFolder(Folder),
    EPlacemark(Placemark),
}

/// a kml document, this is the root of the xml tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub name: String,
    pub description: String,
    pub styles: Vec<Style>,
    pub elements: Vec<Element>,
}

impl From<Placemark> for Element {
    fn from(e: Placemark) -> Self {
        Element::EPlacemark(e)
    }
}

impl From<Point> for GeoElement {
    fn from(p: Point) -> Self {
        GeoElement::EPoint(p)
    }
}

impl Placemark {
    pub fn new_point_on_ground(
        name: String,
        description: String,
        style_id: Option<String>,
        longitude: f64,
        latitude: f64,
    ) -> Self {
        let geo_element: GeoElement = Point {
            longitude,
            latitude,
            altitude: 0f64,
            altitude_mode: AltitudeMode::ErelativeToGround,
        }
        .into();
        Placemark {
            name,
            description,
            geo_element,
            style_id,
            visible: true,
        }
    }
}

impl From<Folder> for Element {
    fn from(e: Folder) -> Self {
        Element::EFolder(e)
    }
}

impl Placemark {
    pub fn new_line(
        name: String,
        description: String,
        style_id: Option<String>,
        points: Vec<(f64, f64)>,
    ) -> Self {
        let points = points
            .iter()
            .map(|(longitude, latitude)| Point {
                longitude: *longitude,
                latitude: *latitude,
                altitude: 0f64,
                altitude_mode: AltitudeMode::ErelativeToGround,
            })
            .collect::<Vec<_>>();
        let geo_element: GeoElement = LineString {
            extrude: 1,
            tessellate: 1,
            altitude_mode: AltitudeMode::ErelativeToGround,
            points,
        }
        .into();
        Placemark {
            name,
            description,
            style_id,
            geo_element,
            visible: true,
        }
    }
}

impl From<LineString> for GeoElement {
    fn from(ls: LineString) -> Self {
        GeoElement::ELineString(ls)
    }
}
