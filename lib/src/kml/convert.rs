use log;
use regex::Regex;

/// parses a position such as provided by google maps/earth,
/// and returns a (lon,lat) tuple
/// # example :
/// ```
/// use lc_gpx_utils::kml::convert::lonlat_of_string ;
/// let s = r###"48°51'29"N 2°17'40"E"### ;
/// let (lon,lat) = lonlat_of_string(s).expect("convert to (long,lat)") ;
///
pub fn lonlat_of_string(s: &str) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    log::info!("{}:{} {}", file!(), line!(), s);
    let re = Regex::new(r###"(\d+)°(\d+)'(\d+)"([N|S]) +(\d+)°(\d+)'(\d+)"([E|W])"###)?;
    let c = re.captures(s);
    log::info!("{}:{} {:?}", file!(), line!(), c);

    let c = c.ok_or("could not parse position string")?;

    let lat = c.get(1).ok_or("1")?.as_str().parse::<f64>()?
        + c.get(2).ok_or("2")?.as_str().parse::<f64>()? / 60.0
        + c.get(3).ok_or("3")?.as_str().parse::<f64>()? / 60.0 / 60.0;
    let lat = match c.get(4).ok_or("4")?.as_str() {
        "N" => lat,
        "S" => -lat,
        _ => return Err("neither N or S".into()),
    };

    let lon = c.get(5).ok_or("5")?.as_str().parse::<f64>()?
        + c.get(6).ok_or("6")?.as_str().parse::<f64>()? / 60.0
        + c.get(7).ok_or("7")?.as_str().parse::<f64>()? / 60.0 / 60.0;
    let lon = match c.get(8).ok_or("8")?.as_str() {
        "W" => -lon,
        "E" => lon,
        _ => return Err("neither E or W".into()),
    };

    log::info!("{}:{} {} {}", file!(), line!(), lon, lat);

    Ok((lon, lat))
}
