use dxf::entities::LwPolyline;
use simple_xml_builder::XMLElement;

pub fn add_lwpolyline(
    lwpolyline: &LwPolyline,
    description: &mut XMLElement,
    lwpolyline_count: &mut u32,
) {
    let mut lwpolyline_xml: XMLElement = XMLElement::new("polygon");
    lwpolyline.vertices.iter().enumerate().for_each(|(j, _i)| {
        lwpolyline_xml.add_attribute(format!("x{}", (j + 1)), lwpolyline.vertices[j].x);
        lwpolyline_xml.add_attribute(format!("y{}", (j + 1)), -lwpolyline.vertices[j].y);
    });
    lwpolyline_xml.add_attribute("closed", "false");
    lwpolyline_xml.add_attribute("antialias", "false");
    if lwpolyline.thickness > 0.1 {
        lwpolyline_xml.add_attribute(
            "style",
            "line-style:normal;line-weight:normal;filling:none;color:black",
        );
    } else {
        lwpolyline_xml.add_attribute(
            "style",
            "line-style:normal;line-weight:thin;filling:none;color:black",
        );
    }
    description.add_child(lwpolyline_xml);
    *lwpolyline_count += 1;
}
