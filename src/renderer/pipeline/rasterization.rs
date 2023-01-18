use crate::general::positions_2d::{Point as Point2, Triangle as Triangle2, paralellogram_area, dot_product_2d};
use crate::general::positions_3d::{Triangle as Triangle3, Point as Point3, dot_product};
use crate::renderer::interface::Buffer;
use crate::renderer::pipeline::fragment_shader::fragment_shader;

pub fn render_triangle(
    ss_triangle: &Triangle3, // screen space triangle
    pixel_buffer: &mut Buffer<f32>,
    depth_buffer: &mut Buffer<f32>,
    light_intensity: Option<f32>
    ) {
    let triangle2 = ss_triangle.to_2d();
    if !triangle2.has_area() {
      return
    }

    let bc_calculator = BarycentricCoordinates::new(&triangle2);

    // make 2d bounding box
    let [min_x, max_x, min_y, max_y] = ss_triangle.get_min_max_x_y();
    let start_x = min_x.floor() as usize;
    let stop_x = (max_x.ceil() + 1.) as usize;
    let start_y = min_y.floor() as usize;
    let stop_y = (max_y.ceil() + 1.) as usize;

    // fill in the correct pixels
    for y in start_y..stop_y {
        for x in start_x..stop_x {
            let point = Point2 { x: x as f32, y: y as f32};
            let [v, w] = bc_calculator.get_coordinates(&point);
            let pixel_is_inside_triangle = point_is_inside_triangle(v, w);
            if pixel_is_inside_triangle {
                continue;
            }
            let triangle_points = ss_triangle.points();
            let frag_depth = triangle_points[0].z + v * (triangle_points[1].z - triangle_points[0].z) + w * (triangle_points[2].z - triangle_points[0].z);
            if frag_depth - 0.01 <= depth_buffer.get(x, y) {
                depth_buffer.set(x, y, frag_depth);
                // TODO triangle should be screen space (-1 to 1), is currently (-width*0.5 to width*0.5)
                if let Some(light_intensity) = light_intensity {
                    pixel_buffer.set(x, y, light_intensity);
                } else {
                    pixel_buffer.set(x, y, fragment_shader(ss_triangle));
                }
            }
        }
    }
}

// from https://ceng2.ktu.edu.tr/~cakir/files/grafikler/Texture_Mapping.pdf
// TODO might only work with 3d points
struct BarycentricCoordinates {
    d00: f32,
    d01: f32,
    d11: f32,
    v0: Point2,
    v1: Point2,
    denominator: f32,
}

impl BarycentricCoordinates {
    fn new(triangle: &Triangle2) -> Self {
        let origin_vertex = &triangle.p1;
        let vertex0 = triangle.p2.relative_to(origin_vertex);
        let vertex1 = triangle.p3.relative_to(origin_vertex);
        let d00 = dot_product_2d(&vertex0, &vertex0);
        let d01 = dot_product_2d(&vertex0, &vertex1);
        let d11 = dot_product_2d(&vertex1, &vertex1);

        BarycentricCoordinates {
            d00,
            d01,
            d11,
            v0: vertex0,
            v1: vertex1,
            denominator: d00 * d11 - d01 * d01
        }
    }

    fn get_coordinates(&self, point: &Point2) -> [f32; 2] {
        let d20 = dot_product_2d(&point, &self.v0);
        let d21 = dot_product_2d(&point, &self.v1);
        [
            (self.d11 * d20 - self.d01 * d21) / self.denominator,
            (self.d00 * d21 - self.d01 * d20) / self.denominator,
        ]
    }
}

// from https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/barycentric-coordinates
///
///  P = p1 + u * (p2 - p1) + v * (p3 - p1)
///
// TODO does not work correctly when point is outside of triangle
fn get_barycentric_coordinates(triangle: &Triangle2, point: &Point2) -> (f32, f32) {
    let p = point.relative_to(&triangle.p1);
    let p2 = triangle.p2.relative_to(&triangle.p1);
    let p3 = triangle.p3.relative_to(&triangle.p1);
    let divisor = paralellogram_area(&p2, &p3);
    let u = paralellogram_area(&p, &p3) / divisor;
    let v = paralellogram_area(&p, &p2) / divisor;
    (u, v)
}

/// Takes barycentric coordinates of the point inside the triangle as parameters
fn point_is_inside_triangle(u: f32, v: f32) -> bool {
    u >= 0. && u <= 1. && v >= 0. && v <= 1. && (u + v) <= 1.
}

#[cfg(test)]
mod test_bc {
    use crate::general::positions_2d::Triangle as Triangle2;
    use crate::general::positions_2d::Point as Point2;
    use crate::renderer::pipeline::rasterization::BarycentricCoordinates;

    #[test]
    fn test_get_barycentric_coordinates_1() {
        let triangle = Triangle2 {
            p1: Point2 { x: -2.0, y: -1.0},
            p2: Point2 { x: -2.0, y: 2.0},
            p3: Point2 { x: 1.0, y: -1.0}
        };
        let point = Point2 {
            x: -0.5,
            y: 0.5
        };
        let calc = BarycentricCoordinates::new(&triangle);
        let result = calc.get_coordinates(&point);
        assert_eq!(result, [0.5, 0.5]);
    }

    #[test]
    fn test_get_barycentric_coordinates_2() {
        let triangle = Triangle2 {
            p1: Point2 { x: 9.0, y: 9.0},
            p2: Point2 { x: 9.0, y: 0.0},
            p3: Point2 { x: 0.0, y: 9.0}
        };
        let point = Point2 {
            x: 9.0,
            y: 0.0
        };
        let calc = BarycentricCoordinates::new(&triangle);
        let result = calc.get_coordinates(&point);
        assert_eq!(result, [1.0, 0.0]);
    }

    #[test]
    fn test_get_barycentric_coordinates_3() {
        let triangle = Triangle2 {
            p1: Point2 { x: -3.0, y: 3.0},
            p2: Point2 { x: -3.0, y: 0.0},
            p3: Point2 { x: 0.0, y: 0.0}
        };
        let calc = BarycentricCoordinates::new(&triangle);

        assert_eq!([0.0, 0.0], calc.get_coordinates(&Point2 { x: -3.0, y: 3.0 }));
        assert_eq!([1.0, 0.0], calc.get_coordinates(&Point2 { x: -3.0, y: 0.0 }));
        assert_eq!([0.0, 1.0], calc.get_coordinates(&Point2 { x: 0.0, y: 0.0 }));

        assert_eq!([-1.0, 1.0], calc.get_coordinates(&Point2 { x: 0.0, y: 3.0 }));
    }
}
