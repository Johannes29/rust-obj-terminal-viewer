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

pub struct BoundingBox(Point, Point);

#[derive(Debug)]
pub struct Degrees(pub f32);

// TODO add two methods for multiplication and addition on each component of the point
// like add_xyz and multiply_xyz, but with only one parameter
impl Point {
    pub fn from_array(array: [f32; 3]) -> Self {
        Point {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }

    pub fn from_vec(array: Vec<f32>) -> Option<Self> {
        if array.len() != 3 {
            return None;
        }
        Some(Point {
            x: array[0],
            y: array[1],
            z: array[2],
        })
    }

    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        let components = self.to_array().map(f);
        Point::from_array(components)
    }

    pub fn combine<F>(&self, point: &Point, f: F) -> Self
    where
        F: Fn(f32, f32) -> f32,
    {
        Point {
            x: f(self.x, point.x),
            y: f(self.y, point.y),
            z: f(self.z, point.z),
        }
    }

    pub fn relative_to(&self, point: &Point) -> Point {
        let closure = |own_component, point_component| own_component - point_component;
        self.combine(point, closure)
    }

    pub fn add(&self, point: &Point) -> Point {
        let closure = |own_component, point_component| own_component + point_component;
        self.combine(point, closure)
    }

    pub fn to_2d(&self) -> Point2 {
        Point2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn normalized(&self) -> Self {
        let length = distance_from_origo(self);
        let closure = |component| component / length;
        self.map(closure)
    }

    pub fn inverted(&self) -> Self {
        let closure = |component: f32| -component;
        self.map(closure)
    }
}

impl Triangle {
    pub fn points(&self) -> [&Point; 3] {
        [&self.p1, &self.p2, &self.p3]
    }

    // TODO the following 4 methods are inconsistent

    // the last element in the array should be the normal
    #[allow(unused)]
    pub fn from_arr_n(array: [Point; 4]) -> Self {
        Triangle {
            p1: array[0].clone(),
            p2: array[1].clone(),
            p3: array[2].clone(),
            normal: array[3].clone(),
        }
    }

    pub fn from_arr(array: [Point; 3]) -> Self {
        Triangle {
            p1: array[0].clone(),
            p2: array[1].clone(),
            p3: array[2].clone(),
            normal: Triangle::get_normal(array),
        }
    }

    /**
     * Calculates normal based on the triangle vert positions in the array argument
     */
    #[allow(unused)]
    pub fn from_vec(array: Vec<Point>) -> Option<Self> {
        if array.len() < 3 {
            return None;
        }
        Some(Triangle {
            p1: array[0].clone(),
            p2: array[1].clone(),
            p3: array[2].clone(),
            normal: Triangle::get_normal([array[0].clone(), array[1].clone(), array[2].clone()]),
        })
    }

    /**
     * Specify normal of triangle with a separate argument
     */
    pub fn from_vec_n(array: Vec<Point>, normal: Point) -> Option<Self> {
        if array.len() < 3 {
            return None;
        }
        Some(Triangle {
            p1: array[0].clone(),
            p2: array[1].clone(),
            p3: array[2].clone(),
            normal,
        })
    }

    pub fn combine_with_point<F>(&self, point: &Point, function: F) -> Self
    where
        F: Fn(&Point, &Point) -> Point,
    {
        let points = self.points().map(|self_point| function(self_point, point));
        Triangle::from_arr(points)
    }

    pub fn add_point(&self, point: &Point) -> Self {
        let closure = |triangle_point: &Point, other_point: &Point| triangle_point.add(other_point);
        self.combine_with_point(point, closure)
    }

    pub fn multiply_with_point(&self, point: &Point) -> Self {
        let closure = |triangle_point: &Point, other_point: &Point| {
            triangle_point.combine(other_point, |a, b| a * b)
        };
        self.combine_with_point(point, closure)
    }

    pub fn to_2d(&self) -> Triangle2 {
        Triangle2 {
            p1: self.p1.to_2d(),
            p2: self.p2.to_2d(),
            p3: self.p3.to_2d(),
        }
    }

    pub fn get_normal(points: [Point; 3]) -> Point {
        let a = points[1].relative_to(&points[0]);
        let b = points[2].relative_to(&points[0]);
        cross_product(a, b).normalized()
    }

    // TODO merge this and the above function
    pub fn get_normal_2(points: &[&Point]) -> Point {
        let a = points[1].relative_to(points[0]);
        let b = points[2].relative_to(points[0]);
        cross_product(a, b).normalized()
    }
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            triangles: Vec::new(),
        }
    }
}

impl BoundingBox {
    fn initialize() -> Self {
        let zero_point = Point {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        BoundingBox(zero_point.clone(), zero_point)
    }

    // TODO DRY
    fn expand(&mut self, point: &Point) {
        if point.x < self.0.x {
            self.0.x = point.x
        }
        if point.y < self.0.y {
            self.0.y = point.y
        }
        if point.z < self.0.z {
            self.0.z = point.z
        }
        if point.x > self.1.x {
            self.1.x = point.x
        }
        if point.y > self.1.y {
            self.1.y = point.y
        }
        if point.z > self.1.z {
            self.1.z = point.z
        }
    }

    pub fn new<'a, T: Iterator<Item = &'a Triangle>>(iter: &mut T) -> Self {
        let mut bounding_box = BoundingBox::initialize();

        while let Some(triangle) = iter.next() {
            for point in triangle.points() {
                bounding_box.expand(point);
            }
        }

        bounding_box
    }

    pub fn get_center(&self) -> Point {
        let get_middle = |point1, point2| (point1 + point2) / 2.0;
        self.0.combine(&self.1, get_middle)
    }

    pub fn get_bounding_radius(&self) -> f32 {
        distance(&self.0, &self.1) / 2.0
    }
}

// #[cfg(test)]
// mod tests {
//     use super::Point as Point3;
// #[test]
// fn get_normal() {
//     let points = [
//         Point3 { x: -1., y: -1., z: 1. },
//         Point3 { x: -1., y: 1., z: 1. },
//         Point3 { x: 1., y: 1., z: 1. }
//     ];
//     assert_eq!(result, 4);
// }
// }

pub fn dot_product(a: &Point, b: &Point) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross_product(a: Point, b: Point) -> Point {
    Point {
        x: (a.y * b.z - b.y * a.z),
        y: (a.x * b.z - b.x * a.z),
        z: (a.x * b.y - b.x * a.y),
    }
}

#[allow(unused)]
pub fn distance(p1: &Point, p2: &Point) -> f32 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dz = p1.z - p2.z;

    (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
}

pub fn distance_from_origo(point: &Point) -> f32 {
    (point.x.powi(2) + point.y.powi(2) + point.z.powi(2)).sqrt()
}
