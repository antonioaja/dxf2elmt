use dxf::entities::*;
use simple_xml_builder::XMLElement;

pub fn add_polyline(polyline: &Polyline, description: &mut XMLElement, polyline_count: &mut u32) {
    let mut polyline_xml: XMLElement = XMLElement::new("polygon");
    polyline.__vertices_and_handles.iter().enumerate().for_each(|(j, _i)| {
        polyline_xml.add_attribute(
            format!("x{}", (j + 1)),
            polyline.__vertices_and_handles[j].0.location.x,
        );
        polyline_xml.add_attribute(
            format!("y{}", (j + 1)),
            -polyline.__vertices_and_handles[j].0.location.y,
        );
    });
    polyline_xml.add_attribute("closed", "false");
    polyline_xml.add_attribute("antialias", "false");
    if polyline.thickness > 0.1 {
        polyline_xml.add_attribute(
            "style",
            "line-style:normal;line-weight:normal;filling:none;color:black",
        );
    } else {
        polyline_xml.add_attribute(
            "style",
            "line-style:normal;line-weight:thin;filling:none;color:black",
        );
    }
    description.add_child(polyline_xml);
    *polyline_count += 1;
}