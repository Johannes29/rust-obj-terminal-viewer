use crate::general::positions_3d::Triangle as Triangle3;

// TODO barycentric coordinates as a parameter
pub fn fragment_shader(
    ss_triangle: &Triangle3, // screen space triangle
    cs_triangle: &Triangle3  // camera space triangle
    ) -> f32 {
    cs_triangle.normal.x
}