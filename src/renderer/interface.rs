use std::{time::{Duration, Instant}, io::{stdout, Write}, thread};
use crossterm::{cursor, ExecutableCommand, terminal, event::Event, event::EnableMouseCapture};
use crate::general::positions_3d::Point as Point3;
use super::events::*;
use super::render::render_mesh;
use super::pipeline::terminal_output::{draw_char_buffer, image_buffer_to_char_buffer};
use crate::general::positions_3d::Mesh;

// TODO should have separate camera struct, with both fov and view_point
pub struct Renderer {
    width: u16,
    height: u16,
    pub horizontal_fov: f32,
    pub vertical_fov: f32,
    pub view_point: Point3,
    chars: Vec<u8>,
    pub mesh: Mesh,
    pub frame_time: Duration,
    pub char_buffer: Vec<Vec<u8>>,
    pub prev_char_buffer: Vec<Vec<u8>>,
    image_buffer: Vec<Vec<f32>>,
    depth_buffer: Vec<Vec<f32>>,
    pub mesh_rotation_x: f32,
    pub mesh_rotation_y: f32,
    pub mesh_rotation_z: f32,
    pub rotation_origin: Point3,
    pub light_direction: Point3,
    pub near: f32,
    pub far: f32,
}

#[allow(dead_code)]
pub enum ShouldExit {
    Yes,
    No,
}

// TODO #anti_aliasing: add parameter for antialiasing sampling (aa: u8), example values: 1 (normal), 2, 4, 8, ...
// TODO take a config struct?
impl Renderer {
    pub fn new(width: u16, height: u16, fps: f32, char_asp_ratio: f32, fov: f32, brightness_string: &str) -> Self {
        let aspect_ratio = height as f32 * char_asp_ratio / width as f32;
        let empty_char_buffer = Renderer::get_empty_char_buffer(width, height);

        Renderer {
            width,
            height,
            horizontal_fov: get_horizontal_fov(fov, aspect_ratio),
            vertical_fov: get_vertical_fov(fov, aspect_ratio),
            // TODO make parameter of new()
            view_point: Point3 { x: 0.0, y: 1.0, z: -8.0 },
            chars: brightness_string.as_bytes().to_vec(),
            mesh: Mesh { triangles: Vec::new(), },
            frame_time: Duration::from_secs_f32(1.0 / fps),
            char_buffer: empty_char_buffer.clone(),
            prev_char_buffer: empty_char_buffer.clone(),
            image_buffer: Renderer::get_empty_image_buffer(width, height),
            depth_buffer: Renderer::get_empty_depth_buffer(width, height),
            mesh_rotation_x: 0.0,
            mesh_rotation_y: 0.0,
            mesh_rotation_z: 0.0,
            // TODO make parameter of new()
            rotation_origin: Point3 { x: 0.0, y: 1.0, z: 0.0 },
            // TODO make parameter of new()
            light_direction: Point3 {x: -0.3, y: 0.5, z: 0.5}.normalized(),
            near: 6.0,
            far: 10.0,
        }
    }

    fn prepare_for_rendering(&self) {
        stdout().execute(cursor::Hide).unwrap();
        terminal::enable_raw_mode().unwrap();
        Renderer::clear_terminal();
        stdout().execute(cursor::MoveTo(0, self.height)).unwrap();
        stdout().execute(EnableMouseCapture).unwrap();
    }

    pub fn render_frame(&mut self) {
        self.clear_image_buffer();
        self.clear_depth_buffer();
        render_mesh(&self.mesh, &mut self.image_buffer, &mut self.depth_buffer, &self.view_point, self.mesh_rotation_x, self.mesh_rotation_y, &self.rotation_origin, &self.light_direction, self.horizontal_fov, self.vertical_fov, self.near, self.far);
        image_buffer_to_char_buffer(&self.image_buffer, &mut self.char_buffer, &self.chars);
        draw_char_buffer(&self.char_buffer, &self.prev_char_buffer);

        self.prev_char_buffer = self.char_buffer.clone();
    }

    pub fn start_rendering<F>(&mut self, mut call_every_frame: F) where F: FnMut(&mut Self, Vec<Event>) -> ShouldExit {
        self.prepare_for_rendering();
        loop {
            let start_time = Instant::now();

            let events = get_events_from_queue();
            if events.iter().any(|event| should_exit(event)) { break }
            let should_exit = call_every_frame(self, events);
            if let ShouldExit::Yes = should_exit { break }
            self.render_frame();

            let end_time = Instant::now();
            let compute_and_draw_time = end_time - start_time;

            match self.frame_time.checked_sub(compute_and_draw_time) {
                Some(duration) => thread::sleep(duration),
                None => (),
            };
        }
        Renderer::after_rendering_stopped();
    }

    fn after_rendering_stopped() {
        stdout().execute(cursor::Show).unwrap();
        terminal::disable_raw_mode().unwrap();
    }

    fn clear_terminal() {
        let terminal_height = terminal::size().unwrap().1;
        // let cursor_pos_y = cursor::position().unwrap().1;
        let empty_lines_to_print = terminal_height - 1;

        stdout().write(&vec![b'\n'; empty_lines_to_print as usize]).unwrap();
        // stdout().execute(terminal::Clear(terminal::ClearType::All));
    }

    fn get_empty_char_buffer(width: u16, height: u16) -> Vec<Vec<u8>> {
        vec![vec![b' '; width as usize]; height as usize]
    }

    fn get_empty_image_buffer(width: u16, height: u16) -> Vec<Vec<f32>> {
        vec![vec![0.0; width as usize]; height as usize]
    }
    
    fn get_empty_depth_buffer(width: u16, height: u16) -> Vec<Vec<f32>> {
        vec![vec![f32::MAX; width as usize]; height as usize]
    }

    fn clear_image_buffer(&mut self) {
        self.image_buffer = Renderer::get_empty_image_buffer(self.width, self.height);
    }

    fn clear_depth_buffer(&mut self) {
        self.depth_buffer = Renderer::get_empty_depth_buffer(self.width, self.height);
    }
}

/// aspect ratio = height / width
fn get_horizontal_fov(diagonal_fov: f32, aspect_ratio: f32) -> f32 {
    let aspect_ratio_angle = aspect_ratio.atan();
    2.0 * ((diagonal_fov.to_radians() / 2.0).tan() * aspect_ratio_angle.cos()).atan().to_degrees()
}

/// aspect ratio = height / width
fn get_vertical_fov(diagonal_fov: f32, aspect_ratio: f32) -> f32 {
    let aspect_ratio_angle = aspect_ratio.atan();
    2.0 * ((diagonal_fov.to_radians() / 2.0).tan() * aspect_ratio_angle.sin()).atan().to_degrees()
}
