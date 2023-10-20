use crate::general::positions_2d::{Point as Point2, Triangle as Triangle2};
use crate::general::positions_3d::Triangle as Triangle3;
use crate::renderer::interface::Buffer;
use crate::renderer::pipeline::fragment_shader::fragment_shader;

pub fn render_triangle(
    ps_triangle: &Triangle3, // pixel space triangle
    // TODO ^ function should take screen space triangle (normalized screen coordinates) instead
    // Then you can check screen space intersection and render with the same triangle
    pixel_buffer: &mut Buffer<f32>,
    depth_buffer: &mut Buffer<f32>,
    light_intensity: Option<f32>,
) {
    let triangle2 = ps_triangle.to_2d();
    if !triangle2.has_area() {
        // return;
    }

    // let bc_calculator = BarycentricCoordinates::new(&triangle2);

    // Using + 2 instead of .roof() + 1 because it's faster
    let [min_x, max_x, min_y, max_y] = ps_triangle.get_min_max_x_y();
    let start_x = min_x as usize;
    let stop_x = max_x as usize + 2;
    let start_y = min_y as usize;
    let stop_y = max_y as usize + 2;

    // fill in the correct pixels
    for y in start_y..stop_y {
        for x in start_x..stop_x {
            let point = Point2 {
                x: x as f32,
                y: y as f32,
            };
            let [w, u, v] = get_barycentric_coordinates(&triangle2, &point, true);
            let pixel_is_inside_triangle = point_is_inside_triangle([w, u, v]);
            if !pixel_is_inside_triangle {
                continue;
            }
            let triangle_points = ps_triangle.points();
            let frag_depth = triangle_points[0].z
                + u * (triangle_points[1].z - triangle_points[0].z)
                + v * (triangle_points[2].z - triangle_points[0].z);

            let Some(depth_buffer_value) = depth_buffer.get(x, y) else {
                // Pixel is outside of the rendered surface
                continue
            };
            if frag_depth <= depth_buffer_value {
                depth_buffer.set(x, y, frag_depth).unwrap();
                // TODO triangle should be screen space (-1 to 1), is currently (-width*0.5 to width*0.5)
                if let Some(light_intensity) = light_intensity {
                    pixel_buffer.set(x, y, light_intensity).unwrap();
                } else {
                    pixel_buffer
                        .set(x, y, fragment_shader(ps_triangle))
                        .unwrap();
                }
            }
        }
    }
}

/// `P = v0 * w0 + v1 * w1 + v2 * w2` where
/// v0, v1, v2 are vertices of triangle, P is point,
/// and \[w0, w1, w2\] the return value of this function
///
/// Assumes clockwise winding order
fn get_barycentric_coordinates(triangle: &Triangle2, point: &Point2, clockwise: bool) -> [f32; 3] {
    let [v0, v1, v2] = triangle.points();
    let p = point;

    let denominator =
        edge_function(&v2.relative_to(v0), &v1.relative_to(v0)) * if clockwise { 1. } else { -1. };
    let w2 = edge_function(&p.relative_to(v0), &v1.relative_to(v0)) / denominator;
    let w0 = edge_function(&p.relative_to(v1), &v2.relative_to(v1)) / denominator;
    let w1 = edge_function(&p.relative_to(v2), &v0.relative_to(v2)) / denominator;

    [w0, w1, w2]
}

/// Takes the barycentric coordinates of the point inside the triangle as a parameter
fn point_is_inside_triangle(barycents: [f32; 3]) -> bool {
    barycents[0] >= 0. && barycents[1] >= 0. && barycents[2] >= 0.
}

/// Same output as converting points to 3d and calling cross_product(p1, p2).
/// This function is hopefully faster
fn edge_function(p1: &Point2, p2: &Point2) -> f32 {
    p1.x * p2.y - p1.y * p2.x
}

#[cfg(test)]
mod test_bc {
    use crate::general::positions_2d::Point as Point2;
    use crate::general::positions_2d::Triangle as Triangle2;
    use crate::renderer::pipeline::rasterization::get_barycentric_coordinates;

    /// bcc => BaryCentric Coordinates
    fn get_point_from_bcc(triangle: &Triangle2, point: &Point2) -> Point2 {
        let [w0, w1, w2] = get_barycentric_coordinates(&triangle, &point, true);
        let mut point_from_barycentric_coordinates = Point2 { x: 0.0, y: 0.0 };
        point_from_barycentric_coordinates.add(&triangle.p1.scale(w0));
        point_from_barycentric_coordinates.add(&triangle.p2.scale(w1));
        point_from_barycentric_coordinates.add(&triangle.p3.scale(w2));
        point_from_barycentric_coordinates
    }

    #[test]
    fn test_get_barycentric_coordinates_1() {
        let triangle = Triangle2 {
            p1: Point2 { x: -2.0, y: 0.0 },
            p2: Point2 { x: -2.0, y: 2.0 },
            p3: Point2 { x: 0.0, y: 0.0 },
        };
        let point = Point2 { x: -1.0, y: 1.0 };
        assert_eq!(point, get_point_from_bcc(&triangle, &point));
    }

    #[test]
    fn test_bc_when_point_outside() {
        let triangle = Triangle2 {
            p1: Point2 { x: -2.0, y: 0.0 },
            p2: Point2 { x: -1.0, y: 3.0 },
            p3: Point2 { x: 1.5, y: 2.0 },
        };
        let point = Point2 { x: 1.0, y: 0.5 };
        assert_eq!(point, get_point_from_bcc(&triangle, &point));
    }
}
