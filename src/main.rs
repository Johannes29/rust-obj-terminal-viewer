mod general;
mod renderer;

use crossterm::event::{KeyCode, KeyModifiers, MouseButton};

use renderer::interface::{Renderer, ShouldExit};
use renderer::obj_parser::parse_obj;

use crossterm::event::Event;
use crossterm::event::MouseEventKind;
use crossterm::terminal;
use crate::general::positions_3d::Point;

// +x is to the right, +y is up, +z is forwards
fn main() {
    let obj_path = "objects/hourglass.obj";
    let terminal_size = terminal::size().unwrap();
    let mut renderer = Renderer::new(terminal_size.0, terminal_size.1, 10.0, 2.0, 80.0, " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$");
    let mesh = match parse_obj(obj_path) {
        Ok(mesh) => mesh,
        Err(message) => {
            println!("Error when parsing {obj_path}: {message}");
            return;
        }
    };
    renderer.mesh = mesh;

    let mut start_column: Option<u16> = None;
    let mut start_row: Option<u16> = None;
    let mut rotation_already_applied = Rotation { around_x: 0., around_y: 0. };
    let mut rotation_origin = Point {
        x: 0.,
        y: 0.,
        z: 0.
    };

    let mut frame_loop = |renderer_todo: &mut Renderer, _events: Vec<Event>| -> ShouldExit {
        for event in _events {
            if let Event::Key(key_event) = event {
                if let KeyCode::Char(char) = key_event.code {
                    match char {
                        'a' => {
                            rotation_origin.x -= 1.0;
                        },
                        'd' => {
                            rotation_origin.x += 1.0;
                        },
                        's' => {
                            rotation_origin.z -= 1.0;
                        },
                        'w' => {
                            rotation_origin.z += 1.0;
                        },
                        'f' => {
                            rotation_origin.y -= 1.0;
                        },
                        'r' => {
                            rotation_origin.y += 1.0;
                        },
                        _ => (),
                    }
                }
            }
            if let Event::Mouse(mouse_event) = event {
                match mouse_event.kind {
                    MouseEventKind::Drag(MouseButton::Middle) => {
                        if mouse_event.modifiers == KeyModifiers::NONE {
                            if let (Option::Some(start_column), Option::Some(start_row)) = (start_column, start_row) {
                                let cell_movement = get_cell_movement(start_column, start_row, mouse_event.column, mouse_event.row);
                                let mut camera_rotation = get_rotation(cell_movement[0], cell_movement[1], terminal_size.0, terminal_size.1, 2.0, 0.5);
                                camera_rotation.around_x += rotation_already_applied.around_x;
                                camera_rotation.around_y += rotation_already_applied.around_y;
                                renderer_todo.camera_rotation_x = camera_rotation.around_x;
                                renderer_todo.camera_rotation_y = camera_rotation.around_y;
                                let camera_position = get_position(camera_rotation, 5.0);
                                renderer_todo.view_point = camera_position.add(&rotation_origin);
                            }
                        }
                    },
                    MouseEventKind::Down(MouseButton::Middle) => {
                        start_column = Some(mouse_event.column);
                        start_row = Some(mouse_event.row);
                        rotation_already_applied = Rotation {
                            around_x: renderer_todo.camera_rotation_x,
                            around_y: renderer_todo.camera_rotation_y
                        };
                    }
                    _ => (),
                }
            }
        }

        ShouldExit::No
    };

    renderer.start_rendering(&mut frame_loop);
}


#[derive(Debug)]
struct Rotation {
    around_x: f32,
    around_y: f32,
}

// calculates rotation x and y from moved x and y
/**
 * char_aspect_ratio is width / height
 */
fn get_rotation(
    cell_movement_x: i32,
    cell_movement_y: i32,
    terminal_width: u16,
    terminal_height: u16,
    rotation_speed: f32,
    char_aspect_ratio: f32
) -> Rotation {
    let x_movement = cell_movement_x as f32 / terminal_width as f32;
    let y_movement = cell_movement_y as f32 / terminal_height as f32;
    let window_aspect_ratio = terminal_width as f32 / terminal_height as f32;
    let rotation_around_x = y_movement as f32 * rotation_speed / char_aspect_ratio;
    let rotation_around_y = x_movement as f32 * rotation_speed * window_aspect_ratio;

    Rotation { around_x: rotation_around_x, around_y: rotation_around_y }
}

/**
 * Returns [x, y], [column, row]
 */
fn get_cell_movement(start_column: u16, start_row: u16, column: u16, row: u16) -> [i32; 2] {
    [
        column as i32 - start_column as i32,
        row as i32 - start_row as i32
    ]
}

fn get_position(rotation: Rotation, radius: f32) -> Point {
    Point {
        x: -rotation.around_y.sin() * radius * rotation.around_x.cos(),
        y: rotation.around_x.sin() * radius,
        z: -rotation.around_y.cos() * radius * rotation.around_x.cos(),
    }
}
