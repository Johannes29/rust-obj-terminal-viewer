use crate::general::positions_3d::Triangle as Triangle3;

// TODO barycentric coordinates as a parameter
pub fn fragment_shader(triangle: &Triangle3) -> f32 {
    triangle.normal.x
}