use crate::general::positions_3d::Triangle as Triangle3;
use crate::general::positions_3d::Point as Point3;

use std::f32::consts::PI;

type Matrix4x1 = [[f32; 1]; 4];
type Matrix4x4 = [[f32; 4]; 4];

trait MatrixTrait {
    fn multiply(&self, _:Matrix4x1) -> Matrix4x1;
}

impl Point3 {
    fn to_matrix4x1(&self) -> Matrix4x1 {
        [
            [self.x],
            [self.y],
            [self.z],
            [1.0]
        ]
    }
}

trait MatrixVector {
    fn to_vec3(self) -> [f32; 3];
}

impl MatrixTrait for Matrix4x4  {
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
pub fn persp_proj_mat(
    vertical_fov_deg: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32
) -> Matrix4x4 {
    let v = vertical_fov_deg * PI / 180.;
    let a = aspect_ratio;
    let n = near;
    let f = far;
    [
        [1./(a*(v/2.).tan()), 0., 0., 0.],
        [0., -1./((v/ 2.).tan()), 0., 0.],
        [0., 0., f/(f-n), -f*n/(f-n)],
        [0., 0., 1., 0.]
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
            // TODO is this inefficient?
            if (-1.0..=1.0).contains(&(k*-1.0 + m))
            || (-1.0..=1.0).contains(&(k*1.0 + m))
            || (-1.0..=1.0).contains(&((-1.0 - m) / k))
            || (-1.0..=1.0).contains(&((1.0 - m) / k)) {
                return true
            }
        }
    }

    false
}

pub fn triangle3d_to_screen_space_triangle(
    triangle3: &Triangle3,
    pp_matrix: Matrix4x4,
    view_point: &Point3
) -> Triangle3 {
    let mut new_points: Vec<Point3> = Vec::new();
    for world_pos in triangle3.points() {
        let pos_matrix = world_pos.relative_to(view_point).to_matrix4x1();
        let new_point = pp_matrix
           .multiply(pos_matrix)
            .to_vec3();

        new_points.push(Point3::from_arr(new_point));
    }
    Triangle3::from_vec_n(new_points, triangle3.normal.clone()).unwrap()
}