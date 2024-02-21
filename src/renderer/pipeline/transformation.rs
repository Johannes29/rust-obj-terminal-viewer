use crate::general::positions_3d::Point as Point3;
use crate::general::positions_3d::Triangle as Triangle3;

type Matrix4x1 = [[f32; 1]; 4];
type Matrix4x4 = [[f32; 4]; 4];

pub trait MatrixTrait {
    fn multiply(&self, _: Matrix4x1) -> Matrix4x1;
    fn combine(&self, _: Matrix4x4) -> Matrix4x4;
}

impl Point3 {
    fn to_matrix4x1(&self) -> Matrix4x1 {
        [[self.x], [self.y], [self.z], [1.0]]
    }
}

trait MatrixVector {
    fn to_vec3(self) -> [f32; 3];
}

impl MatrixTrait for Matrix4x4 {
    fn multiply(&self, other: Matrix4x1) -> Matrix4x1 {
        let mut new_matrix = [[0.0], [0.0], [0.0], [0.0]];
        for row_i in 0..4 {
            let mut sum = 0.0;
            for i in 0..4 {
                let self_val = self[row_i][i];
                let other_val = other[i][0];
                sum += self_val * other_val;
            }
            new_matrix[row_i][0] = sum;
        }

        new_matrix
    }

    fn combine(&self, other: Matrix4x4) -> Matrix4x4 {
        let mut new_matrix = [
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
        ];
        for row_i in 0..4 {
            for col_i in 0..4 {
                let mut sum = 0.0;

                for i in 0..4 {
                    let self_val = self[row_i][i];
                    let other_val = other[i][col_i];
                    sum += self_val * other_val;
                }
                new_matrix[row_i][col_i] = sum;
            }
        }
        new_matrix
    }
}

impl MatrixVector for Matrix4x1 {
    fn to_vec3(self) -> [f32; 3] {
        let w = self[3][0];
        let x = self[0][0] / w;
        let y = self[1][0] / w;
        let z = self[2][0] / w;

        [x, y, z]
    }
}

// from https://austinmorlan.com/posts/rotation_matrices
/// Rotates around the x-axis.
/// NOTE: adapted for right-hand coordinate system
pub fn rotation_matrix_x(angle: f32) -> Matrix4x4 {
    [
        [1., 0., 0., 0.],
        [0., angle.cos(), -angle.sin(), 0.],
        [0., angle.sin(), angle.cos(), 0.],
        [0., 0., 0., 1.],
    ]
}

// from https://austinmorlan.com/posts/rotation_matrices
/// Rotates around the y-axis.
/// NOTE: adapted for right-hand coordinate system
pub fn rotation_matrix_y(angle: f32) -> Matrix4x4 {
    [
        [angle.cos(), 0., angle.sin(), 0.],
        [0., 1., 0., 0.],
        [-angle.sin(), 0., angle.cos(), 0.],
        [0., 0., 0., 1.],
    ]
}

pub fn translation_matrix(x: f32, y: f32, z: f32) -> Matrix4x4 {
    [
        [1., 0., 0., x],
        [0., 1., 0., y],
        [0., 0., 1., z],
        [0., 0., 0., 1.],
    ]
}

pub fn translation_matrix_subtract_point(point: &Point3) -> Matrix4x4 {
    [
        [1., 0., 0., -point.x],
        [0., 1., 0., -point.y],
        [0., 0., 1., -point.z],
        [0., 0., 0., 1.],
    ]
}

pub fn screen_to_pixel_coordinates(screen_width: usize, screen_height: usize) -> Matrix4x4 {
    let w = screen_width as f32;
    let h = screen_height as f32;
    [
        [w / 2., 0., 0., w / 2.],
        [0., h / 2., 0., h / 2.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]
}

// TODO not fitting for this file (transformations)
pub fn triangle_intersects_screen_space(triangle: &Triangle3) -> bool {
    let mut points_inside_screen_space = 0;
    for point in triangle.points() {
        if point.x >= -1. && point.x <= 1. && point.y >= -1. && point.y <= 1. {
            points_inside_screen_space += 1;
        }
    }

    if points_inside_screen_space > 0 {
        return true;
    } else {
        let points = triangle.points();
        for i in 0..3 {
            let p1 = points[i];
            let p2 = points[(i + 1).rem_euclid(3)];

            let k = p1.y - p2.y / (p1.x - p2.x);
            let m = p1.y - k * p1.x;
            match k.is_nan() {
                false => {
                    // TODOO this does not work correctly, acts like the edges of the triangle are infinite.
                    // TODO is this inefficient?
                    if (-1.0..=1.0).contains(&(k * -1.0 + m))
                        || (-1.0..=1.0).contains(&(k * 1.0 + m))
                        || (-1.0..=1.0).contains(&((-1.0 - m) / k))
                        || (-1.0..=1.0).contains(&((1.0 - m) / k))
                    {
                        return true;
                    }
                }
                true => {
                    if (-1.0..=1.0).contains(&k) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

pub fn get_multiplied_points_with_matrix(points: &Vec<Point3>, matrix: &Matrix4x4) -> Vec<Point3> {
    points
        .iter()
        .map(|point| {
            let pos_matrix = point.to_matrix4x1();
            let new_pos_matrix = matrix.multiply(pos_matrix).to_vec3();
            Point3::from_array(new_pos_matrix)
        })
        .collect()
}

#[derive(Debug)]
pub struct Camera {
    pub horizontal_fov: f32,
    pub vertical_fov: f32,
    pub position: Point3,
    pub rotation_around_x: f32,
    pub rotation_around_y: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn world_to_screen_space_matrix(&self) -> Matrix4x4 {
        self.persp_proj_mat()
            .combine(rotation_matrix_x(-self.rotation_around_x))
            .combine(rotation_matrix_y(-self.rotation_around_y))
            .combine(translation_matrix_subtract_point(&self.position))
    }
    // from https://youtu.be/U0_ONQQ5ZNM?t=784 but adapted for right hand coordinate system with -z forwards and +y up
    /// fov in degrees
    fn persp_proj_mat(&self) -> Matrix4x4 {
        let h = self.horizontal_fov.to_radians();
        let v = self.vertical_fov.to_radians();
        let n = self.near;
        let f = self.far;
        [
            [1. / (h / 2.).tan(), 0., 0., 0.],
            [0., -1. / (v / 2.).tan(), 0., 0.],
            [0., 0., -f / (f - n), -f * n / (f - n)],
            [0., 0., -1., 0.],
        ]
    }
}
