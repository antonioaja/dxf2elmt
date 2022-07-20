use dxf::entities::*;
use simple_xml_builder::XMLElement;

pub fn add_solid(solid: &Solid, description: &mut XMLElement, solid_count: &mut u32) {
    let mut solid_xml: XMLElement = XMLElement::new("polygon");
    solid_xml.add_attribute("x1", solid.first_corner.x);
    solid_xml.add_attribute("y1", -solid.first_corner.y);
    solid_xml.add_attribute("x2", solid.second_corner.x);
    solid_xml.add_attribute("y2", -solid.second_corner.y);
    solid_xml.add_attribute("x3", solid.third_corner.x);
    solid_xml.add_attribute("y3", -solid.third_corner.y);
    solid_xml.add_attribute("x4", solid.fourth_corner.x);
    solid_xml.add_attribute("y4", -solid.fourth_corner.y);
    solid_xml.add_attribute("closed", "true");
    solid_xml.add_attribute("antialias", "false");
    if solid.thickness > 0.5 {
        solid_xml.add_attribute(
            "style",
            "line-style:normal;line-weight:normal;filling:none;color:black",
        );
    } else {
        solid_xml.add_attribute(
            "style",
            "line-style:normal;line-weight:thin;filling:none;color:black",
        );
    }
    description.add_child(solid_xml);
    *solid_count += 1;
}