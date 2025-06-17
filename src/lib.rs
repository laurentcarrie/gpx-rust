//! this crate allows you to:
//! - transform a garmin workout tcx file to a polars dataframe
//! - allow you to manipulate this dataframe to :
//!     - have custom plots of your workout
//!     - have custom kml output of your workout, that you can load in google earth

pub mod course;

#[allow(clippy::needless_doctest_main)]
#[allow(clippy::empty_line_after_doc_comments)]
/// # these are utilities to write a kml file from a rust data model
///
/// __warning__ : there is no capability to read a kml file...., the intent here is only to write kml files
/// with a rust data model, no XML. Not all kml features are \[yet\] supported.
///
/// input information is taken from <https://developers.google.com/kml/documentation>
///
/// Simple usage is
/// - Instanciate a [kml::model::Document] rust structure
/// - write it to a kml file with [kml::write::write_kml],
/// # example :
/// ```
/// use std::path::PathBuf ;
/// use lc_gpx_utils::kml::model as M ;
/// use lc_gpx_utils::kml::write::write_kml ;
/// use lc_gpx_utils::kml::convert::lonlat_of_string ;
///
/// fn main() {
///
/// // a first kml placemark
/// let eiffel_tower : M::Placemark = M::Placemark::new_point_on_ground(
///                     "Eiffel Tower".to_string(),
///                     "the Paris icon".to_string(),
///                     // use style-1 for this placemark
///                     Some("style-1".to_string()),
///                     2.2944919,48.8582027
///                 );
/// // a second kml placemark
/// let (lon,lat) = lonlat_of_string(r###"48°51'11"N 2°20'56"E"###).expect("get (lon,lat)") ;
/// let notre_dame : M::Placemark = M::Placemark::new_point_on_ground(
///                     "Notre Dame".to_string(),
///                     "home of Quasimodo".to_string(),
///                     Some("style-1".to_string()),
///                     lon,lat
///                 );
/// // the kml document
/// let doc = M::Document {
///     name:"a name".to_string(),
///     description:"a description".to_string(),
///     styles:vec![
///         // a user defined style
///         M::Style {
///             id: "style-1".to_string(),
///             icon_url: "https://maps.google.com/mapfiles/kml/shapes/trail.png".to_string(),
///             icon_style_scale: 1.0,
///             line_style_width: 1.6,
///             }
///     ],
///     // what is inside the document
///     elements:vec![
///         M::Folder{
///             name:"Paris".to_string(),
///             description:"".to_string(),
///             elements:vec![
///                 eiffel_tower.into(),
///                 notre_dame.into()
///             ]
///         }.into(),
///     ]
/// } ;
/// write_kml(&doc,&PathBuf::from("eiffel-tower.kml")).expect("write kml") ;
/// }
/// ```
///
/// if you load the kml file into google/mymaps, or in google earth, you should get
///  ![Texte alternatif](x.png "Titre de l'image").
pub mod kml;

/// load tcx file to polars dataframe
pub mod tcx;

/// some helpers to transform the workout dataframe
/// a utitilty to a kml document from the dataframe
pub mod transform;
