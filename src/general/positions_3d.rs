use crate::general::positions_2d::Point as Point2;
use crate::general::positions_2d::Triangle as Triangle2;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

// TODO remove unused fill_char
#[derive(Clone, Debug)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub fill_char: u8,
}

#[derive(Debug)]
pub struct Degrees(pub f32);

impl Point {
    pub fn from_arr(array: [f32; 3]) -> Self {
        Point {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }

    pub fn from_vec(array: Vec<f32>) -> Option<Self> {
        if array.len() < 3 {
            return None
        }
        Some(Point {
            x: array[0],
            y: array[1],
            z: array[2],
        })
    }

    pub fn relative_to(&self, point: &Point) -> Point {
        Point {
            x: self.x - point.x,
            y: self.y - point.y,
            z: self.z - point.z,
        }
    }

    pub fn to_2d(&self) -> Point2 {
        Point2 {
            x: self.x,
            y: self.y
        }
    }
}

impl Triangle {
    pub fn points(&self) -> [&Point; 3] {
        [&self.p1, &self.p2, &self.p3]
    }

    pub fn from_arr(array: [Point; 3], fill_char: u8) -> Self {
        Triangle {
            p1: array[0].clone(),
            p2: array[1].clone(),
            p3: array[2].clone(),
            fill_char
        }
    }

    pub fn from_vec(array: Vec<Point>, fill_char: u8) -> Option<Self> {
        if array.len() < 3 {
            return None
        }
        Some(Triangle {
            p1: array[0].clone(),
            p2: array[1].clone(),
            p3: array[2].clone(),
            fill_char
        })
    }

    pub fn add_xyz(&mut self, x: f32, y: f32, z: f32) {
        self.p1 = Point {x: self.p1.x + x, y: self.p1.y + y, z: self.p1.z * z};
        self.p2 = Point {x: self.p2.x + x, y: self.p2.y + y, z: self.p2.z * z};
        self.p3 = Point {x: self.p3.x + x, y: self.p3.y + y, z: self.p3.z * z};
    }
    
    pub fn multiply_xyz(&mut self, x: f32, y: f32, z: f32) {
        self.p1 = Point {x: self.p1.x * x, y: self.p1.y * y, z: self.p1.z * z};
        self.p2 = Point {x: self.p2.x * x, y: self.p2.y * y, z: self.p2.z * z};
        self.p3 = Point {x: self.p3.x * x, y: self.p3.y * y, z: self.p3.z * z};
    }

    pub fn to_2d(&self) -> Triangle2 {
        Triangle2 {
            p1: self.p1.to_2d(),
            p2: self.p2.to_2d(),
            p3: self.p3.to_2d(),
            fill_char: self.fill_char
        }
    }
}

pub fn distance(p1: &Point, p2: &Point) -> f32 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dz = p1.z - p2.z;

    (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
}