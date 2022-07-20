use dxf::entities::*;
use simple_xml_builder::XMLElement;

pub fn add_ellipse(ellipse: &Ellipse, description: &mut XMLElement, ellipse_count: &mut u32) {
    let mut ellipse_xml: XMLElement = XMLElement::new("ellipse");
    ellipse_xml.add_attribute("x", ellipse.center.x - ellipse.major_axis.x);
    ellipse_xml.add_attribute(
        "y",
        -ellipse.center.y - ellipse.major_axis.x * ellipse.minor_axis_ratio,
    );
    ellipse_xml.add_attribute("height", ellipse.major_axis.x * 2.0);
    ellipse_xml.add_attribute(
        "width",
        ellipse.major_axis.x * 2.0 * ellipse.minor_axis_ratio,
    );
    ellipse_xml.add_attribute("antialias", "false");
    ellipse_xml.add_attribute(
        "style",
        "line-style:normal;line-weight:thin;filling:none;color:black",
    );
    description.add_child(ellipse_xml);
    *ellipse_count += 1;
}