use std::{time::{Duration, Instant}, io::{stdout, Write}, thread};
use crossterm::{cursor, ExecutableCommand, terminal, event::Event};
use crate::general::positions_3d::Point as Point3;
use super::events::*;
use super::algorithm::*;
use crate::general::positions_3d::Mesh;

// TODO should have separate camera struct, with both fov and view_point
pub struct Renderer {
    width: u16,
    height: u16,
    horizontal_fov: f32,
    vertical_fov: f32,
    pub view_point: Point3,
    pub mesh: Mesh,
    char_asp_ratio: f32,
    pub frame_time: Duration,
    pub pixel_vec: Vec<Vec<u8>>,
    pub prev_pixel_vec: Vec<Vec<u8>>,
    pub near: f32,
    pub far: f32,
}

#[allow(dead_code)]
pub enum ShouldExit {
    Yes,
    No,
}

impl Renderer {
    pub fn new(width: u16, height: u16, fps: f32, char_asp_ratio: f32, fov: f32) -> Self {
        let mesh = Mesh { triangles: Vec::new(), };
        let pixel_vec = Renderer::get_empty_pixel_vec(width, height);
        let prev_pixel_vec = pixel_vec.clone();
        let frame_time = Duration::from_secs_f32(1.0 / fps);
        let angle_rad = (height as f32 / width as f32).atan();
        let horizontal_fov = fov * angle_rad.cos();
        let vertical_fov = fov * angle_rad.sin();
        let view_point = Point3 { x: 0.0, y: 0.0, z: 0.0 };
        let near = 0.1;
        let far = 100.;

        Renderer {
            width, height, horizontal_fov, vertical_fov, char_asp_ratio,
            view_point, pixel_vec, prev_pixel_vec, mesh, frame_time, near, far
        }
    }

    fn prepare_for_rendering(&self) {
        stdout().execute(cursor::Hide).unwrap();
        Renderer::clear_terminal();
        stdout().execute(cursor::MoveTo(0, self.height)).unwrap();
    }

    fn render_frame(&mut self) {
        self.clear_pixel_vec();
        render_mesh(&self.mesh, &mut self.pixel_vec, self.char_asp_ratio, &self.view_point, self.horizontal_fov, self.vertical_fov, self.near, self.far);
        draw_pixel_array(&self.pixel_vec, &self.prev_pixel_vec);

        self.prev_pixel_vec = self.pixel_vec.clone();
    }

    pub fn start_rendering<F>(&mut self, call_every_frame: F) where F: Fn(&mut Self, Vec<Event>) -> ShouldExit {
        self.prepare_for_rendering();
        loop {
            let start_time = Instant::now();

            let events = get_events_from_queue();
            if events.iter().any(|event| should_exit(event)) { break }
            let should_exit  = call_every_frame(self, events);
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
    }

    fn clear_terminal() {
        let terminal_height = terminal::size().unwrap().1;
        // let cursor_pos_y = cursor::position().unwrap().1;
        let empty_lines_to_print = terminal_height - 1;

        stdout().write(&vec![b'\n'; empty_lines_to_print as usize]).unwrap();
        // stdout().execute(terminal::Clear(terminal::ClearType::All));
    }

    fn get_empty_pixel_vec(width: u16, height: u16) -> Vec<Vec<u8>> {
        vec![vec![b' '; width as usize]; height as usize]
    }

    fn clear_pixel_vec(&mut self) {
        self.pixel_vec = Renderer::get_empty_pixel_vec(self.width, self.height);
    }
}
