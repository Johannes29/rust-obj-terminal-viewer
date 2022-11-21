use std::ops::Mul;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct LinearFunction {
    pub k: f32,
    pub m: f32,
}

#[derive(Debug)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Point {
    #[allow(unused)]
    pub fn add(&mut self, another_point: &Point) {
        self.x += another_point.x;
        self.y += another_point.y;
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

impl LinearFunction {
    pub fn calc(&self, x: f32) -> f32 {
        self.k * x + self.m
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

pub fn get_k(p1: &Point, p2: &Point) -> f32 {
    (p2.y - p1.y) / (p2.x - p1.x)
}


fn paralellogram_area(p1: Point, p2: Point) -> f32 {
    p1.x * p2.y - p1.y * p2.x
}

pub fn get_linear_function(p1: &Point, p2: &Point) -> LinearFunction {
    let k = (p2.y - p1.y) / (p2.x - p1.x);
    let m = p1.y - k * p1.x;
    LinearFunction { k, m } // shorthand for { k: k, m: m}
}