use crate::general::positions_3d::{Point as Point3, Triangle as Triangle3, Mesh, dot_product};
use super::pipeline::rasterization::render_triangle;
use super::pipeline::transformation::{
    persp_proj_mat,
    triangle3d_to_screen_space_triangle,
    triangle_intersects_screen_space, rotation_matrix
};
use image::{GrayImage, Luma};
use super::pipeline::transformation::MatrixTrait;

pub fn render_mesh(
    mesh: &Mesh,
    image_buffer: &mut Vec<Vec<f32>>,
    depth_buffer: &mut Vec<Vec<f32>>,
    view_point: &Point3,
    object_rotation_z: f32,
    light_direction: &Point3,
    horizontal_fov: f32,
    vertical_fov: f32,
    near: f32,
    far: f32
    ) {
    let aspect_ratio = horizontal_fov / vertical_fov;
    let persp_proj_mat = persp_proj_mat(vertical_fov, aspect_ratio, near, far);
    let rotation_matrix = rotation_matrix(0.0, object_rotation_z);
    let transformation_matrix = persp_proj_mat.combine(rotation_matrix);
    let char_buffer_width = image_buffer[0].len() as f32;
    let char_buffer_height = image_buffer.len() as f32;
    let mut triangle_index = 0;
    for world_triangle in &mesh.triangles {
        let mut camera_positions = Vec::new();
        for world_pos in world_triangle.points() {
            camera_positions.push(world_pos.relative_to(view_point))
        }

        // TODO rotate camera here

        let camera_triangle = Triangle3::from_vec_n(camera_positions, world_triangle.normal.clone()).unwrap();

        // TODO should use or instead (||)?
        // Skips triangles behind the camera
        if camera_triangle.p1.z <= 0.0 && camera_triangle.p2.z <= 0.0 && camera_triangle.p3.z <= 0.0 {
            continue;
        }
        let triangle = triangle3d_to_screen_space_triangle(&camera_triangle, transformation_matrix);

        match triangle_intersects_screen_space(&triangle) {
            false => {
                continue;
            },
            true => {
                let mut new_triangle = triangle.clone();

                // screen space to screen coordinate
                new_triangle.add_xyz(1.0, 1.0, 0.0);
                new_triangle.multiply_xyz(0.5, 0.5, 1.0);
                new_triangle.multiply_xyz(
                    char_buffer_width,
                    char_buffer_height,
                    1.0
                );

                // assumes that both normal and light direction are unit vectors
                let light_intensity = dot_product(&triangle.normal, &light_direction.inverted());

                render_triangle(&new_triangle, &camera_triangle, image_buffer, depth_buffer, Some(light_intensity));
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
                triangle_index += 1;
            },
        }
    }
}