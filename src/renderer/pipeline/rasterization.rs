use crate::general::positions_2d::{Point as Point2, get_k, get_linear_function};
use crate::general::positions_3d::{Point as Point3, Triangle as Triangle3, Mesh, dot_product, distance_from_origo};
use super::transformation::{triangle3d_to_screen_space_triangle, persp_proj_mat};
use crate::renderer::pipeline::fragment_shader::fragment_shader;
use std::cmp::{Ordering, min, max};

pub fn render_mesh(mesh: &Mesh, image_buffer: &mut Vec<Vec<f32>>, depth_buffer: &mut Vec<Vec<f32>>, view_point: &Point3, light_direction: &Point3, horizontal_fov: f32, vertical_fov: f32, near: f32, far: f32) {
    let aspect_ratio = horizontal_fov / vertical_fov;
    let persp_proj_mat = persp_proj_mat(vertical_fov, aspect_ratio, near, far);
    let char_buffer_width = image_buffer[0].len() as f32;
    let char_buffer_height = image_buffer.len() as f32;
    for world_triangle in &mesh.triangles {
        let triangle = triangle3d_to_screen_space_triangle(world_triangle, persp_proj_mat, view_point);

        match triangle {
            None => {
                continue;
            },
            Some(mut triangle) => {
                triangle.add_xyz(1.0, 1.0, 0.0);
                triangle.multiply_xyz(0.5, 0.5, 1.0);
                triangle.multiply_xyz(
                    char_buffer_width,
                    char_buffer_height,
                    1.0
                );

                let light_intensity = dot_product(&triangle.normal, light_direction)
                    / (distance_from_origo(&triangle.normal) * distance_from_origo(light_direction));

                // TODO will not work if camera can rotate
                if triangle.normal.z <= 0.0 {
                    render_triangle(&triangle, image_buffer, depth_buffer, Some(light_intensity));
                }
            },
        }
    }
}

pub fn render_triangle(triangle: &Triangle3, pixel_array: &mut Vec<Vec<f32>>, depth_buffer: &mut Vec<Vec<f32>>, light_intensity: Option<f32>) {
    let triangle2 = triangle.to_2d();
    // dbg!(&triangle2);
    if !triangle2.has_area() {
        return
    }

    // dbg!(triangle, pixel_array.len(), pixel_array[0].len());
    let [p1, p2, p3] = triangle2.points();
    #[allow(non_snake_case)]
    let mut pInAscX: [&Point2; 3] = [p1, p2, p3];
    // TODO handle possible None type, dont unwrap
    pInAscX.sort_by(|point_a, point_b| (point_a.x).partial_cmp(&point_b.x).unwrap());

    let (top_edge, bottom_edge) = get_top_and_bottom_edge(p1, p2, p3);

    let start_x = pInAscX[0].x.ceil() as usize;

    let max_x_i = pixel_array[0].len() - 1;
    let max_y_i = pixel_array.len() - 1;

    let start_y_vals = get_y_values_from_edge(bottom_edge, max_x_i, max_y_i);
    let end_y_vals = get_y_values_from_edge(top_edge, max_x_i, max_y_i);

    if start_y_vals.len() != end_y_vals.len() {
        panic!("Impossible!");
    };

    let mut skipped_frags = 0;

    // fill in the correct pixels
    for i in 0..(start_y_vals.len()) {
        let start_y = start_y_vals[i];
        let end_y = end_y_vals[i];
        for y in start_y..end_y {
            let x = start_x + i as usize;
            let y = y as usize;

            // check if x and y are within bounds of pixel_array
            // this should not be necessary anymore, but might as well check
            match pixel_array.get_mut(y) {
                Option::None => continue,
                _ => (),
            }
            match pixel_array[y].get_mut(x) {
                Option::None => break,
                _ => (),
            }
            let frag_depth: f32 = triangle.points().map(|p| p.z).iter().sum::<f32>() / 3.0;
            if frag_depth < depth_buffer[y][x] {
                depth_buffer[y][x] = frag_depth;
                // TODO triangle should be screen space (-1 to 1), is currently (-width*0.5 to width*0.5)
                if let Some(light_intensity) = light_intensity {
                    pixel_array[y][x] = light_intensity;
                } else {
                    pixel_array[y][x] = fragment_shader(triangle);
                }
            } else {
                skipped_frags += 1;
            }
        }
    }

    // print!("{},", skipped_frags)
}

pub fn get_top_and_bottom_edge<'a>(p1: &'a Point2, p2: &'a Point2, p3: &'a Point2) -> (Vec<&'a Point2>, Vec<&'a Point2>) {
    #[allow(non_snake_case)]
    let mut pInAscX: [&Point2; 3] = [p1, p2, p3];
    pInAscX.sort_by(|point_a, point_b| (point_a.x).partial_cmp(&point_b.x).unwrap());
    #[allow(non_snake_case)]
    let mut pInAscY: [&Point2; 3] = [p1, p2, p3];
    pInAscY.sort_by(|point_a, point_b| (point_a.y).partial_cmp(&point_b.y).unwrap());

    let two_vert_edge = vec![pInAscX[0], pInAscX[2]];
    let three_vert_edge = vec![pInAscX[0], pInAscX[1], pInAscX[2]];
    let two_v_k = get_k(pInAscX[0], pInAscX[2]);
    let three_v_k = get_k(pInAscX[0], pInAscX[1]);

    let (mut top_edge, mut bottom_edge) = match two_v_k.partial_cmp(&three_v_k).unwrap() {
        Ordering::Less => (three_vert_edge, two_vert_edge),
        Ordering::Greater => (two_vert_edge, three_vert_edge),
        Ordering::Equal => panic!("Triangle has no area"),
    };

    if pInAscX[0] == pInAscX[2] {
        panic!("Invalid triangle");
    } else if pInAscX[0] == pInAscX[1] {
        let higher_point = if pInAscX[0].y > pInAscX[1].y {pInAscX[0]} else {pInAscX[1]};
        let lower_point =  if pInAscX[0].y > pInAscX[1].y {pInAscX[1]} else {pInAscX[0]};
        let other_point = pInAscX[2];

        top_edge = vec![higher_point, other_point];
        bottom_edge = vec![lower_point, other_point];
    } else if pInAscX[1] == pInAscX[2] {
        let higher_point = if pInAscX[1].y > pInAscX[2].y {pInAscX[1]} else {pInAscX[2]};
        let lower_point =  if pInAscX[1].y > pInAscX[2].y {pInAscX[2]} else {pInAscX[1]};
        let other_point = pInAscX[0];

        top_edge = vec![higher_point, other_point];
        bottom_edge = vec![lower_point, other_point];

    }

    top_edge.sort_by(|point_a, point_b| (point_a.x).partial_cmp(&point_b.x).unwrap());
    bottom_edge.sort_by(|point_a, point_b| (point_a.x).partial_cmp(&point_b.x).unwrap());

    (top_edge, bottom_edge)
}

pub fn get_y_values_from_edge(edge: Vec<&Point2>, max_x_i: usize, max_y_i: usize) -> Vec<usize> {
    let mut y_vals: Vec<usize> = Vec::new();
    
    for i_1 in 0..(edge.len() - 1) {
        let i_2 = i_1 + 1;

        let point_1 = edge[i_1];
        let point_2 = edge[i_2];

        let linear_function = get_linear_function(point_1, point_2);

        let start_x = max(point_1.x.ceil() as i32, 0);
        let end_x = min(point_2.x.ceil() as i32, max_x_i as i32 + 1);

        for x in start_x..end_x {
            let end_y = (linear_function.calc(x as f32) as usize)
                .clamp(0, max_y_i + 1);
            y_vals.push(end_y);
        }
    }

    y_vals
}
