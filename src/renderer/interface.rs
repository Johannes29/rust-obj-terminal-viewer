use super::events::*;
use super::pipeline::terminal_output::{
    add_info_line_to_char_buffer, draw_char_buffer, image_buffer_to_char_buffer,
};
use super::render::render_mesh;
use crate::general::positions_3d::Mesh;
use crate::general::positions_3d::Point as Point3;
use crossterm::{
    cursor,
    event::Event,
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal, ExecutableCommand,
};
use std::{
    io::{stdout, Write},
    thread,
    time::{Duration, Instant},
};

pub use super::pipeline::transformation::Camera;

// TODO should have separate camera struct, with both fov and view_point
pub struct Renderer {
    width: u16,
    height: u16,
    pub camera: Camera,
    chars: Vec<u8>,
    pub mesh: Mesh,
    pub frame_time: Duration,
    pub char_buffer: Buffer<u8>,
    pub prev_char_buffer: Buffer<u8>,
    image_buffer: Buffer<f32>,
    depth_buffer: Buffer<f32>,
    pub info_text: Option<String>,
    pub info_line: String,
    pub light_direction: Point3,
    pub near: f32,
    pub far: f32,
}

pub enum ShouldExit {
    Yes,
    No,
}

// TODO #anti_aliasing: add parameter for antialiasing sampling (aa: u8), example values: 1 (normal), 2, 4, 8, ...
// TODO take a config struct?
impl Renderer {
    pub fn new(
        width: u16,
        height: u16,
        fps: f32,
        char_asp_ratio: f32,
        fov: f32,
        brightness_string: &str,
    ) -> Self {
        let aspect_ratio = height as f32 * char_asp_ratio / width as f32;
        let empty_char_buffer = Buffer::new(width as usize, height as usize, b' ');

        let camera = Camera {
            horizontal_fov: get_horizontal_fov(fov, aspect_ratio),
            vertical_fov: get_vertical_fov(fov, aspect_ratio),
            // TODO make parameter of Renderer::new()
            position: Point3::new(),
            rotation_around_x: 0.0,
            rotation_around_y: 0.0,
            near: 0.01,
            far: 100.0,
        };

        Renderer {
            width,
            height,
            camera,
            chars: brightness_string.as_bytes().to_vec(),
            mesh: Mesh::new(),
            frame_time: Duration::from_secs_f32(1.0 / fps),
            char_buffer: empty_char_buffer.clone(),
            prev_char_buffer: empty_char_buffer.clone(),
            image_buffer: Buffer::new(width as usize, height as usize, 0.0),
            depth_buffer: Buffer::new(width as usize, height as usize, f32::MAX),
            info_text: None,
            info_line: "".to_string(),
            // TODO make parameter of Renderer::new()
            light_direction: Point3 {
                x: -0.3,
                y: -0.5,
                z: -0.5,
            }
            .normalized(),
            near: 6.0,
            far: 10.0,
        }
    }

    pub fn set_mesh(&mut self, mesh: Mesh) {
        self.mesh = mesh;
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
        render_mesh(
            &self.mesh,
            &mut self.image_buffer,
            &mut self.depth_buffer,
            &self.camera,
            &self.light_direction,
        );
        image_buffer_to_char_buffer(&self.image_buffer, &mut self.char_buffer, &self.chars);
        add_info_line_to_char_buffer(&mut self.char_buffer, &self.info_line);
        draw_char_buffer(&self.char_buffer, &self.prev_char_buffer);

        self.prev_char_buffer = self.char_buffer.clone();
    }

    pub fn start_rendering<F>(&mut self, mut call_every_frame: F)
    where
        F: FnMut(&mut Self, Vec<Event>) -> ShouldExit,
    {
        self.prepare_for_rendering();
        loop {
            let start_time = Instant::now();

            let events = get_events_from_queue();
            if events.iter().any(|event| should_exit(event)) {
                break;
            }
            let should_exit = call_every_frame(self, events);
            if let ShouldExit::Yes = should_exit {
                break;
            }
            self.render_frame();

            let end_time = Instant::now();
            let compute_and_draw_time = end_time - start_time;
            self.update_info_line(&compute_and_draw_time);

            match self.frame_time.checked_sub(compute_and_draw_time) {
                Some(duration) => thread::sleep(duration),
                None => (),
            };
        }
        Renderer::after_rendering_stopped();
    }

    fn update_info_line(&mut self, frame_time: &Duration) {
        let frame_time = format!("frame time: {} ms", frame_time.as_micros() as f32 / 1000.0);
        let info_line = match &self.info_text {
            Some(text) => format!("{text} | {frame_time}"),
            None => frame_time,
        };
        self.info_line = info_line;
    }

    fn after_rendering_stopped() {
        stdout().execute(cursor::Show).unwrap();
        terminal::disable_raw_mode().unwrap();
        stdout().execute(DisableMouseCapture).unwrap();
    }

    fn clear_terminal() {
        let terminal_height = terminal::size().unwrap().1;
        let empty_lines_to_print = terminal_height - 1;

        stdout()
            .write_all(&vec![b'\n'; empty_lines_to_print as usize])
            .unwrap();
    }

    fn clear_image_buffer(&mut self) {
        self.image_buffer = Buffer::new(self.width as usize, self.height as usize, 0.0);
    }

    fn clear_depth_buffer(&mut self) {
        self.depth_buffer = Buffer::new(self.width as usize, self.height as usize, f32::MAX);
    }
}

/// aspect ratio = height / width
fn get_horizontal_fov(diagonal_fov: f32, aspect_ratio: f32) -> f32 {
    let aspect_ratio_angle = aspect_ratio.atan();
    2.0 * ((diagonal_fov.to_radians() / 2.0).tan() * aspect_ratio_angle.cos())
        .atan()
        .to_degrees()
}

/// aspect ratio = height / width
fn get_vertical_fov(diagonal_fov: f32, aspect_ratio: f32) -> f32 {
    let aspect_ratio_angle = aspect_ratio.atan();
    2.0 * ((diagonal_fov.to_radians() / 2.0).tan() * aspect_ratio_angle.sin())
        .atan()
        .to_degrees()
}

#[derive(Clone)]
pub struct Buffer<T: Copy> {
    pub values: Vec<T>,
    pub height: usize,
    pub width: usize,
}

impl<T: Copy> Buffer<T> {
    pub fn new(width: usize, height: usize, fill_value: T) -> Self {
        Buffer {
            values: vec![fill_value; width * height],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        match self.get_index(x, y) {
            None => None,
            Some(index) => {
                return Some(self.values[index]);
            }
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), String> {
        match self.get_index(x, y) {
            None => Err("x and y point to a value outside of the buffer".to_string()),
            Some(index) => {
                self.values[index] = value;
                return Ok(());
            }
        }
    }

    /// Returns None if x and y point to a value outside of the buffer
    fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = self.width * y + x;
        if index >= self.height * self.width {
            return None;
        } else {
            return Some(index);
        }
    }
}
