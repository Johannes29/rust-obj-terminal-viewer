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

// from https://youtu.be/U0_ONQQ5ZNM?t=784
/// fov in degrees
pub fn persp_proj_mat(horizontal_fov: f32, vertical_fov: f32, near: f32, far: f32) -> Matrix4x4 {
    let h = horizontal_fov.to_radians();
    let v = vertical_fov.to_radians();
    let n = near;
    let f = far;
    [
        [1. / (h / 2.).tan(), 0., 0., 0.],
        [0., -1. / (v / 2.).tan(), 0., 0.],
        [0., 0., f / (f - n), -f * n / (f - n)],
        [0., 0., 1., 0.],
    ]
}

// from https://austinmorlan.com/posts/rotation_matrices
// NOTE: adapted for left-hand coordinate system
pub fn rotation_matrix_x(angle: f32) -> Matrix4x4 {
    [
        [1., 0., 0., 0.],
        [0., angle.cos(), angle.sin(), 0.],
        [0., -angle.sin(), angle.cos(), 0.],
        [0., 0., 0., 1.],
    ]
}

// from https://austinmorlan.com/posts/rotation_matrices
// NOTE: adapted for left-hand coordinate system
pub fn rotation_matrix_y(angle: f32) -> Matrix4x4 {
    [
        [angle.cos(), 0., -angle.sin(), 0.],
        [0., 1., 0., 0.],
        [angle.sin(), 0., angle.cos(), 0.],
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

pub fn translation_matrix_from_point(point: &Point3) -> Matrix4x4 {
    [
        [1., 0., 0., point.x],
        [0., 1., 0., point.y],
        [0., 0., 1., point.z],
        [0., 0., 0., 1.],
    ]
}

#[cfg(test)]
mod rotation_matrix_tests {
    use super::{rotation_matrix_y, MatrixTrait};
    use std::f32::consts::PI;

    // TODO does not work because floating point errors
    #[test]
    fn test_rotation_matrix_y() {
        let rotation_matrix = rotation_matrix_y(PI / 2.0);
        let point = [[0.5], [2.0], [-3.0], [1.0]];
        let rotated_point = rotation_matrix.multiply(point);
        assert_eq!(rotated_point, [[-3.0], [2.0], [-0.5], [1.0]])
    }
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

pub fn multiply_triangle_points_with_matrix(triangle: &Triangle3, matrix: Matrix4x4) -> Triangle3 {
    let mut new_points: Vec<Point3> = Vec::new();
    for pos in triangle.points() {
        let pos_matrix = pos.to_matrix4x1();
        let new_point = matrix.multiply(pos_matrix).to_vec3();
        new_points.push(Point3::from_array(new_point));
    }
    Triangle3::from_vec_n(new_points, triangle.normal.clone()).unwrap()
}