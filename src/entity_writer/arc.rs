use dxf::entities::*;
use simple_xml_builder::XMLElement;

pub fn add_arc(arc: &Arc, mut _temp: f64, description: &mut XMLElement, arc_count: &mut u32) {
    let mut arc_xml: XMLElement = XMLElement::new("arc");
    arc_xml.add_attribute("x", arc.center.x - arc.radius);
    arc_xml.add_attribute("y", -arc.center.y - arc.radius);
    arc_xml.add_attribute("width", arc.radius * 2.0);
    arc_xml.add_attribute("height", arc.radius * 2.0);
    if arc.start_angle < 0.0 {
        arc_xml.add_attribute("start", -arc.start_angle);
    } else {
        arc_xml.add_attribute("start", arc.start_angle);
    }
    if arc.start_angle > arc.end_angle {
        _temp = (360.0 - arc.start_angle) + arc.end_angle;
    } else {
        _temp = arc.end_angle - arc.start_angle;
    }
    if _temp < 0.0 {
        arc_xml.add_attribute("angle", -_temp);
    } else {
        arc_xml.add_attribute("angle", _temp);
    }
    arc_xml.add_attribute("antialias", "false");
    if arc.thickness > 0.1 {
        arc_xml.add_attribute(
            "style",
            "line-style:normal;line-weight:normal;filling:none;color:black",
        );
    } else {
        arc_xml.add_attribute(
            "style",
            "line-style:normal;line-weight:thin;filling:none;color:black",
        );
    }
    description.add_child(arc_xml);
    *arc_count += 1;
}