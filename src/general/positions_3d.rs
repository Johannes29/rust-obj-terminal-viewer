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

#[derive(Clone, Debug)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub normal: Point,
}

#[derive(Debug)]
pub struct Degrees(pub f32);


// TODO add two methods for multiplication and addition on each component of the point 
// like add_xyz and multiply_xyz, but with only one parameter
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

    pub fn normalized(&self) -> Self {
        let length = distance_from_origo(self);
        Point {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length
        }
    }

    pub fn inverted(&self) -> Self {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Triangle {
    pub fn points(&self) -> [&Point; 3] {
        [&self.p1, &self.p2, &self.p3]
    }

    // TODO the following 4 methods are inconsistent

    // the last element in the array should be the normal
    pub fn from_arr_n(array: [Point; 4]) -> Self {
        Triangle {
            p1: array[0].clone(),
            p2: array[1].clone(),
            p3: array[2].clone(),
            normal: array[3].clone()
        }
    }

    pub fn from_arr(array: [Point; 3]) -> Self {
        Triangle {
            p1: array[0].clone(),
            p2: array[1].clone(),
            p3: array[2].clone(),
            normal: Triangle::get_normal(array)
        }
    }

    /**
     * Calculates normal based on the triangle vert positions in the array argument
     */
    pub fn from_vec(array: Vec<Point>) -> Option<Self> {
        if array.len() < 3 {
            return None
        }
        Some(Triangle {
            p1: array[0].clone(),
            p2: array[1].clone(),
            p3: array[2].clone(),
            normal: Triangle::get_normal([array[0].clone(), array[1].clone(), array[2].clone()])
        })
    }

    /**
     * Specify normal of triangle with a separate argument
     */
    pub fn from_vec_n(array: Vec<Point>, normal: Point) -> Option<Self> {
        if array.len() < 3 {
            return None
        }
        Some(Triangle {
            p1: array[0].clone(),
            p2: array[1].clone(),
            p3: array[2].clone(),
            normal
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
        }
    }
    
    fn get_normal(points: [Point; 3]) -> Point {
        let a = points[1].relative_to(&points[0]);
        let b = points[2].relative_to(&points[0]);
        cross_product(a, b).normalized()
        
    }
}

pub fn dot_product(a: &Point, b: &Point) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross_product(a: Point, b: Point) -> Point {
    Point {
        x: (a.y * b.z - b.y * a.z),
        y: (a.x * b.z - b.x * a.z),
        z: (a.x * b.y - b.x * a.y)
    }
}

pub fn distance(p1: &Point, p2: &Point) -> f32 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dz = p1.z - p2.z;

    (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
}

pub fn distance_from_origo(point: &Point) -> f32 {
    (point.x.powi(2) + point.y.powi(2) + point.z.powi(2)).sqrt()
}