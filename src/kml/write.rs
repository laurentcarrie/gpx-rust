// use itertools::izip;
use log::info;
use std::{fs::File, io::Write, path::PathBuf};

use quick_xml::{
    Writer,
    // events::{BytesCData, BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    events::{BytesCData, BytesDecl, BytesEnd, BytesStart, BytesText, Event},
};

use crate::kml::model as M;

enum ContentType {
    CData,
    Text,
}

fn write_element(
    writer: &mut Writer<Vec<u8>>,
    tag: &str,
    content: &str,
    content_type: ContentType,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_event(Event::Start(BytesStart::new(tag)))?;
    match content_type {
        ContentType::CData => writer.write_event(Event::CData(BytesCData::new(content)))?,
        ContentType::Text => writer.write_event(Event::Text(BytesText::new(content)))?,
    }
    writer.write_event(Event::End(BytesEnd::new(tag)))?;

    Ok(())
}

fn write_point(
    writer: &mut Writer<Vec<u8>>,
    point: &M::Point,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_event(Event::Start(BytesStart::new("Point")))?;
    match &point.altitude_mode {
        M::AltitudeMode::ErelativeToGround => {
            write_element(
                writer,
                "coordinates",
                &format!("{},{},0", point.longitude, point.latitude),
                ContentType::Text,
            )?;
        }
        M::AltitudeMode::Eabsolute => {
            unimplemented!();
        }
    }
    writer.write_event(Event::End(BytesEnd::new("Point")))?;
    Ok(())
}

fn write_linestring(
    writer: &mut Writer<Vec<u8>>,
    ls: &M::LineString,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_event(Event::Start(BytesStart::new("LineString")))?;
    match &ls.altitude_mode {
        M::AltitudeMode::ErelativeToGround => {
            let coordinates = ls
                .points
                .iter()
                .map(|p| format!("{},{},0", p.longitude, p.latitude))
                .collect::<Vec<_>>()
                .join("\n");
            write_element(
                writer,
                "coordinates",
                coordinates.as_str(),
                ContentType::Text,
            )?;
        }
        M::AltitudeMode::Eabsolute => {
            unimplemented!();
        }
    }
    writer.write_event(Event::End(BytesEnd::new("LineString")))?;
    Ok(())
}

fn write_geoelement(
    writer: &mut Writer<Vec<u8>>,
    element: &M::GeoElement,
) -> Result<(), Box<dyn std::error::Error>> {
    match element {
        M::GeoElement::EPoint(p) => write_point(writer, p)?,
        M::GeoElement::ELineString(ls) => write_linestring(writer, ls)?,
    };
    Ok(())
}

fn write_placemark(
    writer: &mut Writer<Vec<u8>>,
    pm: &M::Placemark,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_event(Event::Start(BytesStart::new("Placemark")))?;
    write_element(writer, "name", &pm.name, ContentType::Text)?;
    write_element(writer, "description", &pm.description, ContentType::Text)?;
    if let Some(style_id) = &pm.style_id {
        write_element(
            writer,
            "styleUrl",
            format!("#{}", style_id).as_str(),
            ContentType::Text,
        )?
    };
    write_geoelement(writer, &pm.geo_element)?;
    writer.write_event(Event::End(BytesEnd::new("Placemark")))?;
    Ok(())
}

fn write_style(
    writer: &mut Writer<Vec<u8>>,
    style: &M::Style,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut root = BytesStart::new("Style");
    root.push_attribute(("id", style.id.as_str()));
    writer.write_event(Event::Start(root))?;
    writer.write_event(Event::Start(BytesStart::new("IconStyle")))?;
    write_element(
        writer,
        "scale",
        format!("{}", style.icon_style_scale).as_str(),
        ContentType::Text,
    )?;
    writer.write_event(Event::Start(BytesStart::new("Icon")))?;
    write_element(writer, "href", style.icon_url.as_str(), ContentType::CData)?;
    writer.write_event(Event::End(BytesEnd::new("Icon")))?;
    let mut root = BytesStart::new("hotSpot");
    root.push_attribute(("x", "64"));
    root.push_attribute(("y", "128"));
    root.push_attribute(("xunits", "pixels"));
    root.push_attribute(("yunits", "insetPixels"));
    writer.write_event(Event::End(BytesEnd::new("IconStyle")))?;

    // writer.write_event(Event::Start(BytesStart::new("LineStyle")))?;
    // write_element(writer, "color", "ff2dc0fb", ContentType::Text)?;
    // writer.write_event(Event::End(BytesEnd::new("LineStyle")))?;

    writer.write_event(Event::End(BytesEnd::new("Style")))?;

    // let mut root = BytesStart::new("StyleMap");
    // root.push_attribute(("id", style.id.as_str()));
    // writer.write_event(Event::Start(root))?;
    // writer.write_event(Event::Start(BytesStart::new("Pair")))?;
    // write_element(writer, "key", "normal", ContentType::Text)?;
    // write_element(
    //     writer,
    //     "styleUrl",
    //     format!("__managed_style_{}", style.id).as_str(),
    //     ContentType::Text,
    // )?;
    // writer.write_event(Event::End(BytesEnd::new("Pair")))?;
    // writer.write_event(Event::Start(BytesStart::new("Pair")))?;
    // write_element(writer, "key", "highlight", ContentType::Text)?;
    // write_element(
    //     writer,
    //     "styleUrl",
    //     format!("__{}", style.id).as_str(),
    //     ContentType::Text,
    // )?;
    // writer.write_event(Event::End(BytesEnd::new("Pair")))?;
    // writer.write_event(Event::End(BytesEnd::new("StyleMap")))?;

    Ok(())
}

// <gx:CascadingStyle kml:id="__managed_style_1737FF8B85389E99F6FC">
// 	<Style>
// 		<IconStyle>
// 			<scale>1.2</scale>
// 			<Icon>
// 				<href>https://earth.google.com/earth/document/icon?color=66bb6a&amp;id=2150&amp;scale=4</href>
// 			</Icon>
// 			<hotSpot x="64" y="128" xunits="pixels" yunits="insetPixels"/>
// 		</IconStyle>
// 		<LabelStyle>
// 		</LabelStyle>
// 		<LineStyle>
// 		</LineStyle>
// 		<PolyStyle>
// 		</PolyStyle>
// 		<BalloonStyle>
// 		</BalloonStyle>
// 	</Style>

fn write_folder(
    writer: &mut Writer<Vec<u8>>,
    folder: &M::Folder,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_event(Event::Start(BytesStart::new("Folder")))?;
    write_element(writer, "name", &folder.name, ContentType::Text)?;
    write_element(
        writer,
        "description",
        &folder.description,
        ContentType::Text,
    )?;
    for e in folder.elements.iter() {
        write_doc_element(writer, e)?;
    }
    writer.write_event(Event::End(BytesEnd::new("Folder")))?;
    Ok(())
}

fn write_doc_element(
    writer: &mut Writer<Vec<u8>>,
    element: &M::Element,
) -> Result<(), Box<dyn std::error::Error>> {
    match element {
        M::Element::EPlacemark(p) => write_placemark(writer, p)?,
        M::Element::EFolder(f) => write_folder(writer, f)?,
    }
    Ok(())
}

pub fn write_kml(document: &M::Document, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let buf = Vec::new();
    let mut writer = Writer::new_with_indent(buf, b' ', 4);
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
    let mut root = BytesStart::new("kml");
    root.push_attribute(("xmlns", "http://www.opengis.net/kml/2.2"));
    writer.write_event(Event::Start(root))?;
    writer.write_event(Event::Start(BytesStart::new("Document")))?;
    write_element(&mut writer, "name", &document.name, ContentType::Text)?;

    write_element(
        &mut writer,
        "description",
        &document.description,
        ContentType::Text,
    )?;

    for s in document.styles.iter() {
        write_style(&mut writer, s)?;
    }

    for e in document.elements.iter() {
        log::info!("{}:{} {:?}", file!(), line!(), &e);
        write_doc_element(&mut writer, e)?;
    }

    // let longitudes = vec![2.1, 2.2];
    // let latitudes = vec![48.1, 48.2];
    // let altitudes = vec![0, 0];

    // for (idx, (longitude, latitude, altitude)) in
    //     izip!(longitudes.iter(), latitudes.iter(), altitudes.iter()).enumerate()
    // {
    //     // let altitude = gps_information.get_param("alt");
    //     // let longitude = gps_information.get_param("lon");
    //     // let latitude = gps_information.get_param("lat");
    //     let description = "a description";
    //     writer.write_event(Event::Start(BytesStart::new("Placemark")))?;
    //     write_element(
    //         &mut writer,
    //         "name",
    //         &(idx + 1).to_string(),
    //         ContentType::Text,
    //     )?;
    //     write_element(&mut writer, "description", &description, ContentType::CData)?;
    //     writer.write_event(Event::Start(BytesStart::new("Point")))?;
    //     write_element(
    //         &mut writer,
    //         "coordinates",
    //         &format!("{},{},{}", longitude, latitude, altitude),
    //         ContentType::Text,
    //     )?;
    //     writer.write_event(Event::End(BytesEnd::new("Point")))?;
    //     writer.write_event(Event::End(BytesEnd::new("Placemark")))?;
    // }

    writer.write_event(Event::End(BytesEnd::new("Document")))?;
    writer.write_event(Event::End(BytesEnd::new("kml")))?;

    let result = writer.into_inner();

    let mut f = File::create(path)?;
    f.write_all(&result)?;
    info!(
        "Successfully written {} bytes to file {:?}",
        result.len() + 38,
        path
    );
    Ok(())
}
