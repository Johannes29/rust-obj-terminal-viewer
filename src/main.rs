use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton};

use rust_obj_terminal_viewer::renderer::interface::{Renderer, ShouldExit};
use rust_obj_terminal_viewer::renderer::obj_parser::ObjParser;
use rust_obj_terminal_viewer::general::positions_3d::Point as Point3;
use rust_obj_terminal_viewer::renderer::render::Camera;

use rust_obj_terminal_viewer::general::positions_3d::Point;
use crossterm::event::Event;
use crossterm::event::MouseEventKind;
use crossterm::terminal;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

// +x is to the right, +y is up, +z is forwards
fn main() {
    let args = Cli::parse();
    let obj_path = args.path;
    let terminal_size = terminal::size().unwrap();
    let mut renderer = Renderer::new(
        terminal_size.0,
        terminal_size.1,
        60.0,
        2.0,
        70.0,
        " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$",
    );
    let mesh = match ObjParser::parse_file(&obj_path) {
        Ok(mesh) => mesh,
        Err(message) => {
            let path_string = obj_path.to_str().unwrap();
            println!("Error when parsing {path_string}: {message}");
            return;
        }
    };
    renderer.set_mesh(mesh);

    let mut drag_key_is_down = false;
    let (mut mouse_column, mut mouse_row) = (0, 0);

    let mut drag_rotation = DragRotation::new(terminal_size.1, terminal_size.0, 2.0, 0.5);

    let mut frame_loop = |renderer_todo: &mut Renderer, _events: Vec<Event>| -> ShouldExit {
        for event in _events {
            if let Event::Mouse(mouse_event) = event {
                (mouse_column, mouse_row) = (mouse_event.column, mouse_event.row);
                match mouse_event.kind {
                    MouseEventKind::Drag(MouseButton::Middle)
                    | MouseEventKind::Drag(MouseButton::Left) => {
                        if mouse_event.modifiers == KeyModifiers::NONE {
                            drag_rotation.handle_drag(
                                mouse_event.column,
                                mouse_event.row,
                                &mut renderer_todo.camera,
                            );
                        }
                    }
                    MouseEventKind::Moved => {
                        if drag_key_is_down {
                            drag_rotation.handle_drag(
                                mouse_event.column,
                                mouse_event.row,
                                &mut renderer_todo.camera,
                            );
                        }
                    }
                    MouseEventKind::Down(MouseButton::Middle)
                    | MouseEventKind::Down(MouseButton::Left) => drag_rotation.handle_drag_start(
                        mouse_event.column,
                        mouse_event.row,
                        terminal::size().unwrap(),
                    ),
                    _ => (),
                }
            }
            if let Event::Key(key_event) = event {
                move_point(&mut renderer_todo.camera.position, key_event);
                if key_event.code == KeyCode::Char('c') {
                    drag_key_is_down = !drag_key_is_down;
                }
                if drag_key_is_down {
                    drag_rotation.handle_drag_start(
                        mouse_column,
                        mouse_row,
                        terminal::size().unwrap(),
                    )
                }
            }
        }
        dbg!(&renderer_todo.camera);

        ShouldExit::No
    };

    renderer.start_rendering(&mut frame_loop);
}

fn move_point(point: &mut Point, key_event: KeyEvent) {
    if let KeyCode::Char(char) = key_event.code {
        match char {
            'a' => {
                point.x -= 1.0;
            }
            'd' => {
                point.x += 1.0;
            }
            's' => {
                point.z -= 1.0;
            }
            'w' => {
                point.z += 1.0;
            }
            'f' => {
                point.y -= 1.0;
            }
            'r' => {
                point.y += 1.0;
            }
            _ => (),
        }
    }
}

#[derive(Debug)]
struct Rotation {
    around_x: f32,
    around_y: f32,
}

impl Rotation {
    fn new() -> Self {
        Rotation {
            around_x: 0.0,
            around_y: 0.0,
        }
    }

    fn sum(rotation_1: &Rotation, rotation_2: &Rotation) -> Self {
        Rotation {
            around_x: rotation_1.around_x + rotation_2.around_x,
            around_y: rotation_1.around_y + rotation_2.around_y,
        }
    }
}

#[derive(Debug)]
struct CellPosition {
    row: u16,
    column: u16,
}

impl CellPosition {
    fn new() -> Self {
        CellPosition { row: 0, column: 0 }
    }

    fn relative_xy_to(&self, other: &Self) -> (i16, i16) {
        (
            self.column as i16 - other.column as i16,
            self.row as i16 - other.row as i16,
        )
    }
}

/// drag refers to when you move the mouse with a special modifier pressed, here the middle mouse button
#[derive(Debug)]
struct DragRotation {
    drag_start_pos: CellPosition,
    rotation_before_drag: Rotation, // rotation when drag started
    drag_rotation: Rotation,        // rotation since drag started
    terminal_height: u16,
    terminal_width: u16,
    rotation_speed: f32,
    char_aspect_ratio: f32,
}

impl DragRotation {
    fn new(
        terminal_height: u16,
        terminal_width: u16,
        rotation_speed: f32,
        char_aspect_ratio: f32,
    ) -> Self {
        DragRotation {
            drag_start_pos: CellPosition::new(),
            rotation_before_drag: Rotation::new(),
            drag_rotation: Rotation::new(),
            terminal_height,
            terminal_width,
            rotation_speed,
            char_aspect_ratio,
        }
    }

    /// Uses the right hand coordinate system.
    fn apply_to_camera(&self, camera: &mut Camera, distance: f32) {
        // Assumes that camera is pointing towards +Z when rotation is 0.
        let (rotation_around_x, rotation_around_y) = self.get_rotation_xy();
        let x = rotation_around_y.sin() * rotation_around_x.cos() * distance;
        let y = rotation_around_x.sin() * distance;
        let z = rotation_around_y.cos() * rotation_around_x.cos() * distance;
        camera.position = Point3 { x, y, z };
        let (rotation_x, rotation_y) = self.get_rotation_xy();
        camera.rotation_around_x = rotation_x;
        camera.rotation_around_y = rotation_y;
    }

    fn get_rotation(&self) -> Rotation {
        Rotation::sum(&self.rotation_before_drag, &self.drag_rotation)
    }

    fn get_rotation_xy(&self) -> (f32, f32) {
        let rotation = self.get_rotation();
        (rotation.around_x, rotation.around_y)
    }

    // terminal_dimensions: (columns, rows)
    fn handle_drag_start(
        &mut self,
        current_column: u16,
        current_row: u16,
        terminal_dimensions: (u16, u16),
    ) {
        self.rotation_before_drag = self.get_rotation();
        self.drag_start_pos = CellPosition {
            column: current_column,
            row: current_row,
        };
        self.terminal_height = terminal_dimensions.1;
        self.terminal_width = terminal_dimensions.0;
    }

    fn handle_drag(&mut self, current_column: u16, current_row: u16, camera: &mut Camera) {
        let current_pos = CellPosition {
            column: current_column,
            row: current_row,
        };
        let (relative_x, relative_y) = current_pos.relative_xy_to(&self.drag_start_pos);
        self.update_drag_rotation(relative_x, relative_y);
        // TODO 10.0 should not be hardcoded
        self.apply_to_camera(camera, 10.0);
    }

    fn update_drag_rotation(&mut self, relative_x: i16, relative_y: i16) {
        let x_movement = relative_x as f32 / self.terminal_width as f32;
        let y_movement = relative_y as f32 / self.terminal_height as f32;
        let window_aspect_ratio = self.terminal_width as f32 / self.terminal_height as f32;
        let rotation_around_x = y_movement as f32 * self.rotation_speed / self.char_aspect_ratio;
        let rotation_around_y = -x_movement as f32 * self.rotation_speed * window_aspect_ratio;

        self.drag_rotation = Rotation {
            around_x: rotation_around_x,
            around_y: rotation_around_y,
        }
    }
}