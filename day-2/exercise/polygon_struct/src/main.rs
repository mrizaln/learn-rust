// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::f64::consts::PI;
use std::ops::Add;
use std::option::Option;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        let square = (self.x * self.x + self.y * self.y) as f64;
        square.sqrt()
    }

    pub fn dist(&self, rhs: Self) -> f64 {
        Self {
            x: rhs.x - self.x,
            y: rhs.y - self.y,
        }
        .magnitude()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    pub fn new() -> Self {
        Self { points: vec![] }
    }

    pub fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    pub fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by(|&l, &r| l.x.cmp(&r.x)).copied()
    }

    pub fn iter(&self) -> std::slice::Iter<Point> {
        self.points.iter()
    }

    pub fn length(&self) -> f64 {
        let p = &self.points;

        if p.is_empty() || p.len() == 1 {
            return 0.0;
        }

        let sides_minus_one = p
            .windows(2)
            .map(|w| w[1].dist(w[0]))
            .reduce(|a, e| a + e)
            .unwrap();

        let one_side = p.first().unwrap().dist(*p.last().unwrap());

        sides_minus_one + one_side
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn circumference(&self) -> f64 {
        2.0 * PI * f64::from(self.radius)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(poly) => poly.length(),
            Shape::Circle(circle) => circle.circumference(),
        }
    }
}

impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Self {
        Shape::Polygon(poly)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
