use crate::general::positions_2d::{Point as Point2, Triangle as Triangle2};
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub points: Vec<Point>,
    pub indices_triangles: Vec<IndicesTriangle>,
}

#[derive(Clone, Debug)]
pub struct Triangle<'a> {
    pub p1: &'a Point,
    pub p2: &'a Point,
    pub p3: &'a Point,
    pub normal: &'a Point, // maybe this should have different lifetime
}

#[derive(Clone, Debug)]
pub struct IndicesTriangle {
    pub p1: usize,
    pub p2: usize,
    pub p3: usize,
    pub normal: Point,
}

pub struct BoundingBox(Point, Point);

#[derive(Debug)]
pub struct Degrees(pub f32);

// TODO add two methods for multiplication and addition on each component of the point
// like add_xyz and multiply_xyz, but with only one parameter
impl Point {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

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

impl<'a> Triangle<'a> {
    pub fn from_indices(
        indices_triangle: &'a IndicesTriangle,
        points: &'a Vec<Point>,
    ) -> Option<Self> {
        Some(Triangle {
            p1: points.get(indices_triangle.p1)?,
            p2: points.get(indices_triangle.p2)?,
            p3: points.get(indices_triangle.p3)?,
            normal: &indices_triangle.normal,
        })
    }

    pub fn points(&self) -> [&Point; 3] {
        [&self.p1, &self.p2, &self.p3]
    }

    pub fn to_2d(&self) -> Triangle2 {
        Triangle2 {
            p1: self.p1.to_2d(),
            p2: self.p2.to_2d(),
            p3: self.p3.to_2d(),
        }
    }

    /// Computes and returns the normal of a triangle defined by three points.
    ///
    /// The normal will point towards the viewer if counterclockwise winding
    /// order is used in a right-hand coordinate system,
    /// or if clockwise winding order is used in a left-hand coordinate system.
    ///
    /// # Example
    /// ```
    /// use rust_obj_terminal_viewer::general::positions_3d::Point as Point3;
    /// use rust_obj_terminal_viewer::general::positions_3d::Triangle as Triangle3;
    ///
    /// let vertices = [
    ///     &Point3::from_array([0.0, 0.0, 0.0]),
    ///     &Point3::from_array([1.0, 0.0, 0.0]),
    ///     &Point3::from_array([0.0, 1.0, 0.0]),
    /// ];
    /// let normal = Triangle3::get_normal(&vertices);
    /// assert_eq!(normal, Point3::from_array([0.0, 0.0, 1.0]));
    /// ```
    pub fn get_normal(points: &[&Point; 3]) -> Point {
        let a = points[1].relative_to(points[0]);
        let b = points[2].relative_to(points[0]);
        cross_product(a, b).normalized()
    }

    /// Uses the vertex normals to choose between the two valid normals for the tree vertices.
    /// The order of the vertices does not matter.
    /// # Example
    /// ```
    /// use rust_obj_terminal_viewer::general::positions_3d::Point as Point3;
    /// use rust_obj_terminal_viewer::general::positions_3d::Triangle as Triangle3;
    ///
    /// let vertices = [
    ///     &Point3::from_array([-2.0, 0.0, 0.0]),
    ///     &Point3::from_array([2.0, 0.0, 0.0]),
    ///     &Point3::from_array([0.0, 0.0, -3.0]),
    /// ];
    /// let vertex_normals = [
    ///     &Point3::from_array([-0.2, 1.0, 0.2]).normalized(),
    ///     &Point3::from_array([0.2, 1.0, 0.2]).normalized(),
    ///     &Point3::from_array([0.0, 1.0, -0.2]).normalized(),
    /// ];
    /// let expected_normal = Point3::from_array([0.0, 1.0, 0.0]);
    /// let returned_normal = Triangle3::get_normal_with_vertex_normals(&vertices, &vertex_normals);
    /// assert_eq!(returned_normal, expected_normal);
    ///
    /// let vertices_flipped_order = [vertices[2], vertices[1], vertices[0]];
    /// let returned_normal_2 = Triangle3::get_normal_with_vertex_normals(&vertices_flipped_order, &vertex_normals);
    /// assert_eq!(returned_normal_2, expected_normal);
    /// ```
    pub fn get_normal_with_vertex_normals(
        vertices: &[&Point; 3],
        vertex_normals: &[&Point; 3],
    ) -> Point {
        let computed_normal = Self::get_normal(vertices);
        let average_vertex_normal = vertex_normals
            .iter()
            .fold(Point::new(), |acc, normal| acc.add(&normal.normalized()))
            .map(|component| component / 3.0);

        if dot_product(&computed_normal, &average_vertex_normal) < 0.0 {
            computed_normal.inverted()
        } else {
            computed_normal
        }
    }

    pub fn get_min_max_x_y(&self) -> [f32; 4] {
        let points = self.points();
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        for point in points {
            let x = point.x;
            let y = point.y;
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }

        [min_x, max_x, min_y, max_y]
    }
}

impl IndicesTriangle {
    pub fn triangle_points<'a>(&self, points: &'a Vec<Point>) -> [&'a Point; 3] {
        [&points[self.p1], &points[self.p2], &points[self.p3]]
    }

    pub fn make_clockwise(&mut self, points: &Vec<Point>) {
        // TODO use dot product of self.normal to determine the correct order of verts
        // then change order of verts
        // optionally also set normal to more precise (probably don't)
        if Triangle::get_normal(&self.triangle_points(points)) == self.normal {
            return;
        }

        let p3_clone = self.p3;
        self.p3 = self.p2;
        self.p2 = p3_clone;
    }
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            points: Vec::new(),
            indices_triangles: Vec::new(),
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
        // Do we really need to clone here?
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

    // TODO maybe move public functions to top of impl block?
    pub fn new(points: &Vec<Point>) -> Self {
        let mut bounding_box = BoundingBox::initialize();
        for point in points {
            bounding_box.expand(point);
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

    #[rustfmt::skip]
    pub fn get_corner_points(&self) -> [Point; 8] {
        [
            Point { x: self.0.x, y: self.0.y, z: self.0.z},
            Point { x: self.0.x, y: self.0.y, z: self.1.z},
            Point { x: self.0.x, y: self.1.y, z: self.0.z},
            Point { x: self.0.x, y: self.1.y, z: self.1.z},
            Point { x: self.1.x, y: self.0.y, z: self.0.z},
            Point { x: self.1.x, y: self.0.y, z: self.1.z},
            Point { x: self.1.x, y: self.1.y, z: self.0.z},
            Point { x: self.1.x, y: self.1.y, z: self.1.z},
        ]
    }

    pub fn get_longest_distance_from_point(&self, point: &Point) -> f32 {
        self.get_corner_points()
            .iter()
            .map(|corner| distance(corner, point))
            .reduce(f32::max)
            .unwrap()
    }
}

pub fn dot_product(a: &Point, b: &Point) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross_product(a: Point, b: Point) -> Point {
    Point {
        x: (a.y * b.z - a.z * b.y),
        y: (a.z * b.x - a.x * b.z),
        z: (a.x * b.y - a.y * b.x),
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

#[cfg(test)]
mod tests {
    use super::BoundingBox;
    use crate::general::positions_3d::{cross_product, distance, Point as Point3};

    #[test]
    fn test_cross_product() {
        let a = Point3::from_array([3.0, 0.0, 0.0]);
        let b = Point3::from_array([0.0, 3.0, 0.0]);
        let cross_product = cross_product(a, b);
        assert_eq!(cross_product, Point3::from_array([0.0, 0.0, 9.0]));
    }

    #[test]
    fn test_cross_product_2() {
        let a = Point3::from_array([1.0, 0.0, 0.0]);
        let b = Point3::from_array([0.0, 0.0, 1.0]);
        let cross_product = cross_product(a, b);
        assert_eq!(cross_product, Point3::from_array([0.0, -1.0, 0.0]));
    }

    #[test]
    fn test_cross_product_should_be_anticommutative() {
        let a = Point3::from_array([0.3, 5.2, 4.5]);
        let b = Point3::from_array([7.4, 0.8, 6.2]);
        let cross_product_1 = cross_product(a.clone(), b.clone());
        let cross_product_2 = cross_product(b, a).inverted();
        assert_eq!(cross_product_1, cross_product_2);
    }

    #[test]
    fn test_get_longest_distance_from_point() {
        let furthest_point = Point3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let bounding_box = BoundingBox(
            furthest_point.clone(),
            Point3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        );
        let origin = Point3::new();
        assert_eq!(
            bounding_box.get_longest_distance_from_point(&origin),
            distance(&origin, &furthest_point),
        );
    }
}
