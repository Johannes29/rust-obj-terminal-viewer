use crate::general::positions_3d::{Point as Point3, Triangle as Triangle3, Mesh, dot_product};
use super::pipeline::rasterization::render_triangle;
use super::pipeline::transformation::{
    persp_proj_mat,
    triangle3d_to_screen_space_triangle,
    triangle_intersects_screen_space, rotation_matrix_x, rotation_matrix_y
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
    // the matrices are combined is equal to if you would first apply the leftmost matrix to the vector,
    // then the one to the right of that one, etc. 
    let transformation_matrix = persp_proj_mat.combine(rotation_matrix_x).combine(rotation_matrix_y);
    let char_buffer_width = image_buffer[0].len() as f32;
    let char_buffer_height = image_buffer.len() as f32;
    let mut triangle_index = 0;

    for world_triangle in &mesh.triangles {
        let mut camera_positions = Vec::new();
        for world_pos in world_triangle.points() {
            camera_positions.push(world_pos.relative_to(view_point))
        }

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
                // triangle_index += 1;
            },
        }
    }
}