use dxf::entities::Line;
use simple_xml_builder::XMLElement;

pub fn add_line(line: &Line, description: &mut XMLElement, line_count: &mut u32) {
    let mut line_xml: XMLElement = XMLElement::new("line");
    line_xml.add_attribute("x1", line.p1.x);
    line_xml.add_attribute("y1", -line.p1.y);
    line_xml.add_attribute("length1", 1.5);
    line_xml.add_attribute("end1", "none");
    line_xml.add_attribute("x2", line.p2.x);
    line_xml.add_attribute("y2", -line.p2.y);
    line_xml.add_attribute("length2", 1.5);
    line_xml.add_attribute("end2", "none");
    line_xml.add_attribute("antialias", "false");
    if line.thickness > 0.5 {
        line_xml.add_attribute(
            "style",
            "line-style:normal;line-weight:normal;filling:none;color:black}",
        );
    } else {
        line_xml.add_attribute(
            "style",
            "line-style:normal;line-weight:thin;filling:none;color:black",
        );
    }
    description.add_child(line_xml);
    *line_count += 1;
}
