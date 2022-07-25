use dxf::entities::*;
use simple_xml_builder::XMLElement;
use min_max::*;


pub fn add_line(line: &Line, description: &mut XMLElement, line_count: &mut u32, min: &mut [i32], max: &mut [i32], first_entity: bool) {
    if first_entity{
        min[0] = line.p1.x as i32;
        min[1] = -line.p1.y as i32;
        max[0] = line.p1.x as i32;
        max[1] = -line.p1.y as i32;
    }
    
    min[0] = min!(min[0], line.p1.x as i32, line.p2.x as i32);
    min[1] = min!(min[1], -line.p1.y as i32, -line.p2.y as i32);
    max[0] = max!(max[0], line.p1.x as i32, line.p2.x as i32);
    max[1] = max!(max[1], -line.p1.y as i32, -line.p2.y as i32);
    
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