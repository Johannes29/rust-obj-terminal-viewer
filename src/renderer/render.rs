use super::interface::Buffer;
use super::pipeline::rasterization::render_triangle;
use super::pipeline::transformation::{MatrixTrait, translation_matrix_from_point};
use super::pipeline::transformation::{
    multiply_triangle_points_with_matrix, persp_proj_mat, rotation_matrix_x, rotation_matrix_y,
    translation_matrix, triangle_intersects_screen_space,
};
use crate::general::positions_3d::{dot_product, Mesh, Point as Point3};
use image::{GrayImage, Luma};

// TODO pass self or settings struct, can not have this many parameters
pub fn render_mesh(
    mesh: &Mesh,
    image_buffer: &mut Buffer<f32>,
    depth_buffer: &mut Buffer<f32>,
    view_point: &Point3,
    mesh_rotation_x: f32,
    mesh_rotation_y: f32,
    rotation_origin: &Point3,
    light_direction: &Point3,
    horizontal_fov: f32,
    vertical_fov: f32,
    near: f32,
    far: f32,
) {
    let persp_proj_mat = persp_proj_mat(horizontal_fov, vertical_fov, near, far);
    let rotation_matrix_x = rotation_matrix_x(mesh_rotation_x);
    let rotation_matrix_y = rotation_matrix_y(mesh_rotation_y);
    let translation_matrix = translation_matrix(-view_point.x, -view_point.y, -view_point.z);
    let pre_rotation_translation = translation_matrix_from_point(&rotation_origin.inverted());
    let post_rotation_translation = translation_matrix_from_point(rotation_origin);
    // the matrices are combined is equal to if you would first apply the leftmost matrix to the vector,
    // then the one to the right of that one, etc.
    let transformation_matrix = persp_proj_mat
        .combine(translation_matrix)
        .combine(post_rotation_translation)
        .combine(rotation_matrix_x)
        .combine(rotation_matrix_y)
        .combine(pre_rotation_translation);
    let mut triangle_index = 0;

    for world_triangle in &mesh.triangles {
        // TODO return Option, None if triangle is outside of frustum
        let triangle = multiply_triangle_points_with_matrix(world_triangle, transformation_matrix);

        // Skips triangles behind the camera
        // TODO use near instead of 0.0
        // TODO do this in triangle3d_to_screen_space_triangle function instead
        if triangle.p1.z <= 0.0 && triangle.p2.z <= 0.0 && triangle.p3.z <= 0.0 {
            continue;
        }

        match triangle_intersects_screen_space(&triangle) {
            false => {
                continue;
            }
            true => {
                // screen space to screen coordinate
                // TODO this should be done in render_triangle function
                let new_triangle = triangle
                    .clone()
                    .add_point(&Point3 {
                        x: 1.,
                        y: 1.,
                        z: 0.,
                    })
                    .multiply_with_point(&Point3 {
                        x: 0.5,
                        y: 0.5,
                        z: 1.,
                    })
                    .multiply_with_point(&Point3 {
                        x: image_buffer.width as f32,
                        y: image_buffer.height as f32,
                        z: 1.0,
                    });

                // assumes that both normal and light direction are unit vectors
                let light_intensity = dot_product(&triangle.normal, &light_direction.inverted());

                render_triangle(
                    &new_triangle,
                    image_buffer,
                    depth_buffer,
                    Some(light_intensity),
                );

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
    }
}
