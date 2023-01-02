use crate::general::positions_3d::{Point as Point3, Mesh, dot_product};
use super::pipeline::rasterization::render_triangle;
use super::pipeline::transformation::{
    persp_proj_mat,
    triangle3d_to_screen_space_triangle,
    triangle_intersects_screen_space, rotation_matrix_x, rotation_matrix_y, translation_matrix
};
use image::{GrayImage, Luma};
use super::pipeline::transformation::MatrixTrait;

pub fn render_mesh(
    mesh: &Mesh,
    image_buffer: &mut Vec<Vec<f32>>,
    depth_buffer: &mut Vec<Vec<f32>>,
    view_point: &Point3,
    view_point_rotation_x: f32,
    view_point_rotation_y: f32,
    light_direction: &Point3,
    horizontal_fov: f32,
    vertical_fov: f32,
    near: f32,
    far: f32
    ) {
    let aspect_ratio = horizontal_fov / vertical_fov;
    let persp_proj_mat = persp_proj_mat(vertical_fov, aspect_ratio, near, far);
    let rotation_matrix_x = rotation_matrix_x(view_point_rotation_x);
    let rotation_matrix_y = rotation_matrix_y(view_point_rotation_y);
    let translation_matrix = translation_matrix(-view_point.x, -view_point.y, -view_point.z);
    // the matrices are combined is equal to if you would first apply the leftmost matrix to the vector,
    // then the one to the right of that one, etc. 
    let transformation_matrix = persp_proj_mat.combine(translation_matrix).combine(rotation_matrix_x).combine(rotation_matrix_y);
    let char_buffer_width = image_buffer[0].len() as f32;
    let char_buffer_height = image_buffer.len() as f32;
    let mut triangle_index = 0;

    for world_triangle in &mesh.triangles {
        // TODO return Option, None if triangle is outside of frustum
        let triangle = triangle3d_to_screen_space_triangle(&world_triangle, transformation_matrix);

        // Skips triangles behind the camera
        // TODO use near instead of 0.0
        // TODO do this in triangle3d_to_screen_space_triangle function instead
        if triangle.p1.z <= 0.0 && triangle.p2.z <= 0.0 && triangle.p3.z <= 0.0 {
            continue;
        }

        match triangle_intersects_screen_space(&triangle) {
            false => {
                continue;
            },
            true => {
                // screen space to screen coordinate
                // TODO this should be done in render_triangle function
                let new_triangle = triangle
                    .clone()
                    .add_point(&Point3 {x: 1., y: 1., z: 0.})
                    .multiply_with_point(&Point3 {x: 0.5, y: 0.5, z: 1.})
                    .multiply_with_point(&Point3 {
                        x: char_buffer_width,
                        y: char_buffer_height,
                        z: 1.0
                    });

                // assumes that both normal and light direction are unit vectors
                let light_intensity = dot_product(&triangle.normal, &light_direction.inverted());

                // TODO remove usage of camera_triangle, 
                render_triangle(&new_triangle, image_buffer, depth_buffer, Some(light_intensity));
                // let height = image_buffer.len() as u32;
                // let width = image_buffer[0].len() as u32;
                // let mut img = GrayImage::new(width, height);
                // for x in 0..width {
                //     for y in 0..height {
                //         img.put_pixel(x, y, Luma([(image_buffer[y as usize][x as usize] * 255.0) as u8]));
                //     }
                // }
                // img.save(format!("frame_{}.png", triangle_index)).unwrap();
                // let mut depth_img = GrayImage::new(width, height);
                // for x in 0..width {
                //     for y in 0..height {
                //         depth_img.put_pixel(x, y, Luma([(depth_buffer[y as usize][x as usize] * 20.0) as u8]));
                //     }
                // }
                // depth_img.save(format!("frame_{}_depth.png", triangle_index)).unwrap();
                // triangle_index += 1;
            },
        }
    }
}