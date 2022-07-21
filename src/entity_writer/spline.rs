extern crate bspline;

use dxf::entities::*;
use simple_xml_builder::XMLElement;
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}
impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn add_spline(spline: &Spline, description: &mut XMLElement, spline_count: &mut u32) {
    let mut i: usize = 0;
    let mut points: Vec<Point> = Vec::new();
    for _a in &spline.control_points {
        points.push(Point::new(
            spline.control_points[i].x,
            spline.control_points[i].y,
        ));
        i += 1;
    }
    i = 0;
    let mut knots: Vec<f64> = Vec::new();
    for _a in &spline.knot_values {
        knots.push(spline.knot_values[i]);
        i += 1;
    }
    let curr_spline =
        bspline::BSpline::new(spline.degree_of_curve.try_into().unwrap(), points, knots);
    let step: f64 = (curr_spline.knot_domain().1 - curr_spline.knot_domain().0) / 100.0;
    let mut spline_xml = XMLElement::new("polygon");
    let mut j: f64 = curr_spline.knot_domain().0;
    i = 0;
    while j < curr_spline.knot_domain().1 {
        spline_xml.add_attribute(format!("x{}", (i + 1)), curr_spline.point(j).x);
        spline_xml.add_attribute(format!("y{}", (i + 1)), -curr_spline.point(j).y);
        j += step;
        i += 1;
    }
    spline_xml.add_attribute("closed", "false");
    spline_xml.add_attribute("antialias", "false");
    spline_xml.add_attribute(
        "style",
        "line-style:normal;line-weight:thin;filling:none;color:black",
    );
    description.add_child(spline_xml);
    *spline_count += 1;
}
