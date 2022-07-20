use dxf::entities::*;
use simple_xml_builder::XMLElement;

pub fn add_circle(circle: &Circle, description: &mut XMLElement, circle_count: &mut u32) {
    let mut circle_xml: XMLElement = XMLElement::new("ellipse");
    circle_xml.add_attribute("x", circle.center.x - circle.radius);
    circle_xml.add_attribute("y", -circle.center.y - circle.radius);
    circle_xml.add_attribute("height", circle.radius * 2.0);
    circle_xml.add_attribute("width", circle.radius * 2.0);
    circle_xml.add_attribute("antialias", "false");
    if circle.thickness > 0.5 {
        circle_xml.add_attribute(
            "style",
            "line-style:normal;line-weight:normal;filling:none;color:black",
        );
    } else {
        circle_xml.add_attribute(
            "style",
            "line-style:normal;line-weight:thin;filling:none;color:black",
        );
    }
    description.add_child(circle_xml);
    *circle_count += 1;
}