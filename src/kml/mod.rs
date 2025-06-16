#[allow(clippy::needless_doctest_main)]
/// the kml data model
/// source information is taken from <https://developers.google.com/kml/documentation>
///
/// this is a set of rust struct that models a kml tree. Not all kml features are \[yet\] supported.
///
/// Simple usage is
/// - Instanciate a [model::Document] rust structure
/// - write it to a kml file with [write::write_kml],
/// # example :
/// ```
/// use std::path::PathBuf ;
/// use lc_gpx_utils::kml::model as M ;
/// use lc_gpx_utils::kml::write::write_kml ;
/// use lc_gpx_utils::kml::convert::lonlat_of_string ;
///
/// fn main() {
///
/// let eiffel_tower : M::Placemark = M::Placemark::new_point_on_ground(
///                     "Eiffel Tower".to_string(),
///                     "the Paris icon".to_string(),
///                     Some("style-1".to_string()),
///                     2.2944919,48.8582027
///                 );
/// let (lon,lat) = lonlat_of_string(r###"48°51'11"N 2°20'56"E"###).expect("get (lon,lat)") ;
/// let notre_dame : M::Placemark = M::Placemark::new_point_on_ground(
///                     "Notre Dame".to_string(),
///                     "home of Quasimodo".to_string(),
///                     Some("style-1".to_string()),
///                     lon,lat
///                 );
/// let doc = M::Document {
///     name:"a name".to_string(),
///     description:"a description".to_string(),
///     styles:vec![
///         M::Style {
///             id: "style-1".to_string(),
///             icon_url: "https://maps.google.com/mapfiles/kml/shapes/trail.png".to_string(),
///             icon_style_scale: 1.0,
///             line_style_width: 1.6,
///             }
///     ],
///     elements:vec![
///         M::Folder{
///             name:"Paris".to_string(),
///             description:"".to_string(),
///             elements:vec![eiffel_tower.into()]
///         }.into(),
///         notre_dame.into()
///     ]
/// } ;
/// write_kml(&doc,&PathBuf::from("eiffel-tower.kml")).expect("write kml") ;
/// }
/// ```
///

/// the kml data model
pub mod model;

/// writes a kml document to .kml file
pub mod write;

pub mod convert;
