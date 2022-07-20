extern crate dxf;
extern crate simple_xml_builder;

use dxf::entities::*;
use dxf::Drawing;
use simple_xml_builder::*;
use std::env;
use std::time::*;

pub mod elmt_writer;
pub mod entity_writer;
pub mod file_writer;

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
    let mut out_file = file_writer::create_file(verbose_output, file_name).unwrap();

    // Definition defintion ;)
    let mut definition = elmt_writer::set_definition();

    // Create uuid
    elmt_writer::set_uuid(&mut definition);

    // Define names
    elmt_writer::set_names(file_name, &mut definition);

    // Define information
    elmt_writer::set_information(&mut definition);

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
    elmt_writer::end_elmt(definition, description, &mut out_file);

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
        file_writer::verbose_print(out_file);
    }

    Ok(())
}
