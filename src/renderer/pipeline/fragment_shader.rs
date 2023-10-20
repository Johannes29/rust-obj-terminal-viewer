use crate::general::positions_3d::Triangle as Triangle3;

// TODO barycentric coordinates as a parameter
pub fn fragment_shader(ss_triangle: &Triangle3, // screen space triangle
) -> f32 {
    ss_triangle.normal.x // TODO
}
