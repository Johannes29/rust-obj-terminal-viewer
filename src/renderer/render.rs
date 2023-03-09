use super::interface::Buffer;
use super::pipeline::rasterization::render_triangle;
use super::pipeline::transformation::get_multiplied_points_with_matrix;
use super::pipeline::transformation::{screen_to_pixel_coordinates, MatrixTrait};
use crate::general::positions_3d::{dot_product, Mesh, Point as Point3, Triangle as Triangle3};
use image::{GrayImage, Luma};

pub fn render_mesh(
    mesh: &Mesh,
    image_buffer: &mut Buffer<f32>,
    depth_buffer: &mut Buffer<f32>,
    camera: &Camera,
    light_direction: &Point3,
) {
    let world_to_screen = camera.world_to_screen_space_matrix();
    let screen_to_pixel = screen_to_pixel_coordinates(image_buffer.width, image_buffer.height);
    // the matrices are combined is equal to if you would first apply the leftmost matrix to the vector,
    // then the one to the right of that one, etc.
    let transformation_matrix = screen_to_pixel.combine(world_to_screen);

    let screen_space_points =
        get_multiplied_points_with_matrix(&mesh.points, &transformation_matrix);

    let mut triangle_index = 0;
    for incides_triangle in &mesh.indices_triangles {
        let triangle = Triangle3::from_indices(incides_triangle, &screen_space_points).unwrap();
        // Skips triangles behind the camera
        // TODO use near instead of 0.0
        // TODO do this in triangle3d_to_screen_space_triangle function instead
        if triangle.p1.z <= 0.0 && triangle.p2.z <= 0.0 && triangle.p3.z <= 0.0 {
            continue;
        }
        // assumes that both normal and light direction are unit vectors
        let light_intensity = dot_product(&triangle.normal, &light_direction.inverted());

        // TODO fix backface culling
        // Does not work currently because normals are not recalculated when rotating mesh.
        // Maybe rotate camera instead of mesh, then no need to recalculate all triangle normals every frame
        // if dot_product(&triangle.normal, &view_direction) >= 0.0 {
        //     continue
        // }

        render_triangle(&triangle, image_buffer, depth_buffer, Some(light_intensity));

        // --- uncomment to generate debug images ---
        //
        // let height = image_buffer.len() as u32;
        // let width = image_buffer[0].len() as u32;
        // let mut img = GrayImage::new(width, height);
        // for x in 0..width {
        //     for y in 0..height {
        //         img.put_pixel(x, y, Luma([(image_buffer[y as usize][x as usize] * 255.0) as u8]));
        //     }
        // }
        // img.save(format!("debug_images/frame_{}.png", triangle_index)).unwrap();
        // let mut depth_img = GrayImage::new(width, height);
        // for x in 0..width {
        //     for y in 0..height {
        //         depth_img.put_pixel(x, y, Luma([(depth_buffer[y as usize][x as usize] * 20.0) as u8]));
        //     }
        // }
        // depth_img.save(format!("debug_images/frame_{}_depth.png", triangle_index)).unwrap();
        // triangle_index += 1;
    }
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
