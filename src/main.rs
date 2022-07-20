extern crate bspline;
extern crate dxf;
extern crate simple_xml_builder;
extern crate tempfile;

use dxf::entities::*;
use dxf::Drawing;
use simple_xml_builder::*;
use std::env;
use std::fs::*;
use std::io::SeekFrom;
use std::io::*;
use std::time::*;
use tempfile::tempfile;
use uuid::*;

pub mod entity_writer;

fn main() -> dxf::DxfResult<()> {
    // Start recording time
    let now = Instant::now();

    // Collect file name argument
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("No file name given.");
    }
    let file_name: &str = &format!("{}", args[1]);

    // Check whether no .elmt is requested
    let mut verbose_output: bool = false;
    if args.len() == 3 {
        if args[2] == "-v" {
            verbose_output = true;
        }
    }

    // Load dxf file
    let drawing: Drawing = Drawing::load_file(file_name)?;
    if !verbose_output {
        println!("{} loaded...", file_name);
    }

    // Intialize counts
    let mut circle_count: u32 = 0;
    let mut line_count: u32 = 0;
    let mut arc_count: u32 = 0;
    let mut spline_count: u32 = 0;
    let mut text_count: u32 = 0;
    let mut ellipse_count: u32 = 0;
    let mut polyline_count: u32 = 0;
    let mut lwpolyline_count: u32 = 0;
    let mut solid_count: u32 = 0;
    let mut other_count: u32 = 0;
    let mut _temp: f64 = 0.0;

    // Create output file for .elmt
    let mut out_file = create_file(verbose_output, file_name)?;

    // Definition defintion ;)
    let mut definition = set_definition();

    // Create uuid
    set_uuid(&mut definition);

    // Define names
    set_names(file_name, &mut definition);

    // Define information
    set_information(&mut definition);

    // Start description
    let mut description: XMLElement = XMLElement::new("description");

    // Loop through all entities, appending to xml file
    drawing.entities().for_each(|e| match e.specific {
        EntityType::Circle(ref circle) => {
            entity_writer::circle::add_circle(circle, &mut description, &mut circle_count);
        }
        EntityType::Line(ref line) => {
            entity_writer::line::add_line(line, &mut description, &mut line_count);
        }
        EntityType::Arc(ref arc) => {
            entity_writer::arc::add_arc(arc, _temp, &mut description, &mut arc_count);
        }
        EntityType::Spline(ref spline) => {
            entity_writer::spline::add_spline(spline, &mut description, &mut spline_count);
        }
        EntityType::Text(ref text) => {
            entity_writer::text::add_text(text, e, &mut description, &mut text_count);
        }
        EntityType::Ellipse(ref ellipse) => {
            entity_writer::ellipse::add_ellipse(ellipse, &mut description, &mut ellipse_count);
        }
        EntityType::Polyline(ref polyline) => {
            entity_writer::polyline::add_polyline(polyline, &mut description, &mut polyline_count);
        }
        EntityType::LwPolyline(ref lwpolyline) => {
            entity_writer::lwpolyline::add_lwpolyline(
                lwpolyline,
                &mut description,
                &mut lwpolyline_count,
            );
        }
        EntityType::Solid(ref solid) => {
            entity_writer::solid::add_solid(solid, &mut description, &mut solid_count);
        }
        _ => {
            other_count += 1;
        }
    });

    // Write to output file
    definition.add_child(description);
    definition.write(&mut out_file)?;

    if !verbose_output {
        println!("Conversion complete!\n");

        // Print stats
        println!("STATS");
        println!("~~~~~~~~~~~~~~~");
        println!("Circles: {}", circle_count);
        println!("Lines: {}", line_count);
        println!("Arcs: {}", arc_count);
        println!("Splines: {}", spline_count);
        println!("Texts: {}", text_count);
        println!("Ellipses: {}", ellipse_count);
        println!("Polylines: {}", polyline_count);
        println!("LwPolylines: {}", lwpolyline_count);
        println!("Solids: {}", solid_count);
        println!("Currently Unsupported: {}", other_count);

        println!("\nTime Elapsed: {} ms", now.elapsed().as_millis());
    } else {
        out_file.seek(SeekFrom::Start(0)).unwrap();

        let mut v_contents = String::new();
        out_file.read_to_string(&mut v_contents).unwrap();
        print!("{}", v_contents);
    }

    Ok(())
}

fn set_information(definition: &mut XMLElement) {
    let mut information: XMLElement = XMLElement::new("informations");
    information.add_text("Created using dxf2elmt!");
    definition.add_child(information);
}

fn set_names(file_name: &str, definition: &mut XMLElement) {
    let mut names: XMLElement = XMLElement::new("names");
    let mut name = XMLElement::new("name");
    name.add_attribute("lang", "en");
    name.add_text(format!("{}", &file_name[0..file_name.len() - 4]));
    names.add_child(name);
    definition.add_child(names);
}

fn set_uuid(definition: &mut XMLElement) {
    let mut uuid: XMLElement = XMLElement::new("uuid");
    uuid.add_attribute("uuid", format!("{{{}}}", Uuid::new_v4()));
    definition.add_child(uuid);
}

fn set_definition() -> XMLElement {
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

fn create_file(verbose_output: bool, file_name: &str) -> Result<File> {
    let mut out_file = tempfile()?;
    if !verbose_output {
        out_file = File::create(format!("{}.elmt", &file_name[0..file_name.len() - 4])).unwrap();
        println!(
            "{}.elmt was created... \nNow converting {}...",
            &file_name[0..file_name.len() - 4],
            file_name
        );
    }
    Ok(out_file)
}
