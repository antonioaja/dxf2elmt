use uuid::*;
use simple_xml_builder::*;
use std::fs::File;

pub fn set_information(definition: &mut XMLElement) {
    let mut information: XMLElement = XMLElement::new("informations");
    information.add_text("Created using dxf2elmt!");
    definition.add_child(information);
}

pub fn set_names(file_name: &str, definition: &mut XMLElement) {
    let mut names: XMLElement = XMLElement::new("names");
    let mut name = XMLElement::new("name");
    name.add_attribute("lang", "en");
    name.add_text(format!("{}", &file_name[0..file_name.len() - 4]));
    names.add_child(name);
    definition.add_child(names);
}

pub fn set_uuid(definition: &mut XMLElement) {
    let mut uuid: XMLElement = XMLElement::new("uuid");
    uuid.add_attribute("uuid", format!("{{{}}}", Uuid::new_v4()));
    definition.add_child(uuid);
}

pub fn set_definition() -> XMLElement {
    let mut definition: XMLElement = XMLElement::new("definition");
    definition.add_attribute("height", 10);
    definition.add_attribute("width", 10);
    definition.add_attribute("hotspot_x", 0);
    definition.add_attribute("hotspot_y", 0);
    definition.add_attribute("version", "0.80");
    definition.add_attribute("link_type", "simple");
    definition.add_attribute("type", "element");
    definition
}

pub fn end_elmt(mut definition: XMLElement, description: XMLElement, out_file: &mut File) {
    definition.add_child(description);
    definition.write(out_file).unwrap();
}