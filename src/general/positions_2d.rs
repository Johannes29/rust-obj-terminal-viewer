use std::ops::Mul;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Point {
    pub fn add(&mut self, another_point: &Point) {
        self.x += another_point.x;
        self.y += another_point.y;
    }

    pub fn scale(&self, number: f32) -> Point {
        Point {
            x: self.x * number,
            y: self.y * number,
        }
    }

    pub fn relative_to(&self, point: &Point) -> Self {
        Point {
            x: self.x - point.x,
            y: self.y - point.y
        }
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Point) -> Self {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl Triangle {    
    pub fn points(&self) -> [&Point; 3] {
        [&self.p1, &self.p2, &self.p3]
    }

    pub fn has_area(&self) -> bool {
        !(self.p1 == self.p2 || self.p2 == self.p3 || self.p3 == self.p1
            || (self.p1.x == self.p2.x && self.p2.x == self.p3.x)
            || (self.p1.y == self.p2.y && self.p2.y == self.p3.y))

    }
}