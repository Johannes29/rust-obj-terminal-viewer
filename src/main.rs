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
    let mut renderer = Renderer::new(terminal_size.0, terminal_size.1, 10.0, 2.0, 110.0, " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$");
    let mesh = match parse_obj(obj_path) {
        Ok(mesh) => mesh,
        Err(message) => {
            println!("Error when parsing {obj_path}: {message}");
            return;
        }
    };
    renderer.mesh = mesh;

    let mut start_row_col: Option<(u16, u16)> = None;
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
                            if let Option::Some(start_row_col) = start_row_col {
                                let mut camera_rotation = get_rotation(start_row_col.0, start_row_col.1, mouse_event.row, mouse_event.column, 0.01, 0.01);
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
                        start_row_col = Some((mouse_event.row, mouse_event.column));
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

fn get_rotation(start_row: u16, start_column: u16, row: u16, column: u16, rad_per_row: f32, rad_per_column: f32)
 -> Rotation {
    let rel_row = row as i16 - start_row as i16;
    let rel_column = column as i16 - start_column as i16;
    let rotation_x = rel_row as f32 * rad_per_row;
    let rotation_y = rel_column as f32 * rad_per_column;

    Rotation { around_x: rotation_x, around_y: rotation_y }
}

fn get_position(rotation: Rotation, radius: f32) -> Point {
    Point {
        x: -rotation.around_y.sin() * radius * rotation.around_x.cos(),
        y: rotation.around_x.sin() * radius,
        z: -rotation.around_y.cos() * radius * rotation.around_x.cos(),
    }
}
