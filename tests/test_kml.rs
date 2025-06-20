#[cfg(test)]
mod tests {
    use assert_float_eq::assert_f64_near;
    use lc_gpx_utils::kml::model as M;
    use lc_gpx_utils::kml::write::write_kml;
    use log::LevelFilter;
    use std::path::PathBuf;

    #[test]
    fn test_placemark() {
        let _ = simple_logging::log_to_file("/tmp/foo.log", LevelFilter::Info)
            .expect("Failed to initialize logging");
        log::info!("start test 3");
        let style_id = "1".to_string();
        let elements: Vec<M::Element> = vec![
            M::Placemark::new_point_on_ground(
                "A".to_string(),
                "first mark".to_string(),
                Some(style_id.clone()),
                2.0,
                48.0,
            )
            .into(),
            M::Placemark::new_point_on_ground(
                "B".to_string(),
                "second mark".to_string(),
                Some("2".to_string()),
                3.0,
                48.0,
            )
            .into(),
            M::Placemark::new_line(
                "XXX".to_string(),
                "a line".to_string(),
                Some("1".to_string()),
                vec![(2.1, 48.1), (2.2, 49.99)],
            )
            .into(),
        ];
        let doc = M::Document {
            name: "test".to_string(),
            description: "some document".to_string(),
            styles: styles(),
            elements,
        };
        write_kml(&doc, &PathBuf::from("test-placemark.kml")).expect("write kml");
        log::info!("{}:{} {:?}", file!(), line!(), doc);
        log::info!("END of test_3");
    }

    fn styles() -> Vec<M::Style> {
        let styles = vec![
            M::Style {
                id: "1".to_string(),
                icon_url:
                    "https://earth.google.com/earth/document/icon?color=66bb6a&id=2150&scale=4"
                        .to_string(),
                icon_style_scale: 1.0,
                line_style_width: 1.6,
            },
            M::Style {
                id: "2".to_string(),
                icon_url: "https://maps.google.com/mapfiles/kml/shapes/trail.png".to_string(),
                icon_style_scale: 1.0,
                line_style_width: 1.6,
            },
        ];
        styles
    }
    #[test]
    fn test_folder() {
        let _ = simple_logging::log_to_file("/tmp/foo.log", LevelFilter::Info)
            .expect("Failed to initialize logging");
        log::info!("start test 3");
        let style_id = "1".to_string();
        let elements: Vec<M::Element> = vec![
            M::Folder {
                name: "xxx".to_string(),
                description: "some desc".to_string(),
                elements: vec![
                    M::Placemark::new_point_on_ground(
                        "A".to_string(),
                        "first mark".to_string(),
                        Some(style_id.clone()),
                        2.0,
                        48.0,
                    )
                    .into(),
                    M::Placemark::new_point_on_ground(
                        "B".to_string(),
                        "second mark".to_string(),
                        Some(style_id.clone()),
                        3.0,
                        48.0,
                    )
                    .into(),
                ],
            }
            .into(),
        ];
        let doc = M::Document {
            name: "test".to_string(),
            description: "some document".to_string(),
            styles: styles(),
            elements,
        };
        write_kml(&doc, &PathBuf::from("test-folder.kml")).expect("write kml");
        log::info!("{}:{} {:?}", file!(), line!(), doc);
        log::info!("END of test_3");
    }

    #[test]
    fn test_folder_in_folder() {
        let _ = simple_logging::log_to_file("/tmp/foo.log", LevelFilter::Info)
            .expect("Failed to initialize logging");
        log::info!("start test 3");
        let style_id = "1".to_string();
        let elements: Vec<M::Element> = vec![
            M::Folder {
                name: "xxx".to_string(),
                description: "yyy".to_string(),
                elements: vec![
                    M::Placemark::new_point_on_ground(
                        "A".to_string(),
                        "first mark".to_string(),
                        Some(style_id.clone()),
                        2.0,
                        48.0,
                    )
                    .into(),
                    M::Placemark::new_point_on_ground(
                        "B".to_string(),
                        "second mark".to_string(),
                        Some("2".to_string()),
                        3.0,
                        48.0,
                    )
                    .into(),
                    M::Folder {
                        name: "inner".to_string(),
                        description: "desc inner".to_string(),
                        elements: vec![],
                    }
                    .into(),
                ],
            }
            .into(),
        ];
        let doc = M::Document {
            name: "test".to_string(),
            description: "some document".to_string(),
            styles: styles(),
            elements,
        };
        write_kml(&doc, &PathBuf::from("test-folder-in-folder.kml")).expect("write kml");
        log::info!("{}:{} {:?}", file!(), line!(), doc);
        log::info!("END of test_3");
    }

    #[test]
    fn test_convert() {
        let _ = simple_logging::log_to_file("/tmp/foo.log", LevelFilter::Info)
            .expect("Failed to initialize logging");
        log::info!("test convert");

        let notre_dame = r###"48°51'11"N 2°20'56"E"###;
        let (lon, lat) = lc_gpx_utils::kml::convert::lonlat_of_string(notre_dame).expect("convert");
        log::info!("{} {}", lon, lat);
        assert_f64_near!(lon, 2.3488888888888892);
        assert_f64_near!(lat, 48.85305555555556);
    }
}
