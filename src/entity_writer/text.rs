use dxf::entities::*;
use simple_xml_builder::XMLElement;

pub fn add_text(text: &Text, e: &Entity, description: &mut XMLElement, text_count: &mut u32) {
    let mut text_xml: XMLElement = XMLElement::new("text");
    text_xml.add_attribute("x", text.location.x);
    text_xml.add_attribute("y", -text.location.y);
    if text.rotation.abs().round() as i64 % 360 != 0 {
        text_xml.add_attribute("rotation", text.rotation - 180.0);
    } else {
        text_xml.add_attribute("rotation", 0);
    }
    text_xml.add_attribute("color", format!("{:x}", e.common.color_24_bit));
    let mut _tmp = &text.text_style_name[..];
    if _tmp == "STANDARD" {
        _tmp = "Arial Narrow";
    }
    text_xml.add_attribute("text", &text.value[..]);
    text_xml.add_attribute(
        "font",
        format!(
            "{},{},-1,5,0,0,0,0,0,0,normal",
            _tmp,
            text.text_height.ceil()
        ),
    );
    description.add_child(text_xml);
    *text_count += 1;
}