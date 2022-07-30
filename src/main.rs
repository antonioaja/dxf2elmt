extern crate dxf;
extern crate simple_xml_builder;

use anyhow::*;
use clap::Parser;
use dxf::entities::EntityType;
use dxf::Drawing;
use simple_xml_builder::XMLElement;
use std::time::*;

#[derive(Parser, Debug)]
#[clap(author, version, about = "A CLI program to convert .dxf files into .elmt files", long_about = None)]
struct Args {
    /// The .dxf file to convert
    #[clap(short, long, value_parser)]
    file_name: String,

    /// Activates verbose output, eliminates .elmt file writing
    #[clap(short, long, value_parser, default_value_t = false)]
    verbose: bool,

    /// Converts text entities into dynamic text instead of the default text box
    #[clap(short, long, value_parser, default_value_t = false)]
    dtext: bool,

    /// Determine the number of lines you want each spline to have (more lines = greater resolution)
    #[clap(short, long, value_parser, default_value_t = 100)]
    spline_step: u32,
}

pub mod elmt_writer;
pub mod entity_writer;
pub mod file_writer;

fn main() -> Result<()> {
    // Start recording time
    let now: Instant = Instant::now();

    // Collect arguments
    let args: Args = Args::parse();
    let file_name: &String = &args.file_name;
    let verbose_output: bool = args.verbose;
    let dtext: bool = args.dtext;
    let spline_step: u32 = args.spline_step;

    // Load dxf file
    let drawing: Drawing = Drawing::load_file(file_name).context(format!(
        "Failed to load {}...\n\tMake sure the file is a valid .dxf file.",
        file_name
    ))?;
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
            entity_writer::arc::add_arc(arc, &mut description, &mut arc_count);
        }
        EntityType::Spline(ref spline) => {
            entity_writer::spline::add_spline(
                spline,
                &mut description,
                &mut spline_count,
                spline_step,
            );
        }
        EntityType::Text(ref text) => {
            entity_writer::text::add_text(text, e, &mut description, &mut text_count, dtext);
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

    // Begin creating .elmt file
    let mut definition = elmt_writer::set_definition();
    elmt_writer::set_uuid(&mut definition);
    elmt_writer::set_names(file_name, &mut definition);
    elmt_writer::set_information(&mut definition);

    // Create output file for .elmt
    let mut out_file = file_writer::create_file(verbose_output, file_name)
        .context("Failed to create output file.")?;

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
        file_writer::verbose_print(out_file).context("Failed to print output to terminal.")?;
    }

    Ok(())
}
