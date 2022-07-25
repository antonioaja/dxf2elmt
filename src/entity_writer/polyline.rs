use dxf::entities::*;
use min_max::*;
use simple_xml_builder::XMLElement;

pub fn add_polyline(
    polyline: &Polyline,
    description: &mut XMLElement,
    polyline_count: &mut u32,
    min: &mut [i32],
    max: &mut [i32],
    first_entity: bool,
) {
    if first_entity {
        min[0] = polyline.__vertices_and_handles[0].0.location.x as i32;
        min[1] = -polyline.__vertices_and_handles[0].0.location.y as i32;
        max[0] = polyline.__vertices_and_handles[0].0.location.x as i32;
        max[1] = -polyline.__vertices_and_handles[0].0.location.y as i32;
    }
    let mut polyline_xml: XMLElement = XMLElement::new("polygon");
    polyline
        .__vertices_and_handles
        .iter()
        .enumerate()
        .for_each(|(j, _i)| {
            polyline_xml.add_attribute(
                format!("x{}", (j + 1)),
                polyline.__vertices_and_handles[j].0.location.x,
            );
            polyline_xml.add_attribute(
                format!("y{}", (j + 1)),
                -polyline.__vertices_and_handles[j].0.location.y,
            );

            min[0] = min!(
                min[0],
                polyline.__vertices_and_handles[j].0.location.x as i32
            );
            min[1] = min!(
                min[1],
                -polyline.__vertices_and_handles[j].0.location.y as i32
            );
            max[0] = max!(
                max[0],
                polyline.__vertices_and_handles[j].0.location.x as i32
            );
            max[1] = max!(
                max[1],
                -polyline.__vertices_and_handles[j].0.location.y as i32
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
