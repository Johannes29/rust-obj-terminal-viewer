mod general;
mod renderer;
use crossterm::event::KeyCode;
use general::positions_3d::Point as Point3;

use general::positions_3d::Mesh;
use general::positions_3d::Triangle;
use renderer::interface::{Renderer, ShouldExit};

use std::vec;
use crossterm::event::Event;
use crossterm::terminal;

// x is to the right, y is down, z is forwards
// TODO positive y should be up
fn main() {
    let terminal_size = terminal::size().unwrap();
    let mut renderer = Renderer::new(terminal_size.0, terminal_size.1, 10.0, 2.0, 70.0, vec![b' ', b'.', b'*', b'#']);
    let mesh = Mesh {
    triangles: vec![
        Triangle {
            p1: Point3 { x: -1.0, y: 0.0, z: 3.0 },
            p3: Point3 { x: -1.0, y: 1.0, z: 4.0 },
            p2: Point3 { x: -1.0, y: 0.0, z: 5.0 },
            fill_char: b'*',
        },
        Triangle {
            p1: Point3 { x: 1.0, y: 0.0, z: 3.0 },
            p3: Point3 { x: 1.0, y: -1.0, z: 4.0 },
            p2: Point3 { x: 1.0, y: 0.0, z: 5.0 },
            fill_char: b'*',
        }
    ]};
    renderer.mesh = mesh;

    let frame_loop = |renderer_todo: &mut Renderer, _events: Vec<Event>| -> ShouldExit {
        // let seconds = start_time.elapsed().as_secs() as f32 + start_time.elapsed().subsec_micros() as f32 / 10_f32.powf(6.0);
        // let angle = seconds  * 360.0 * rps;

        // let mut mesh = original_mesh.clone();
        // mesh.triangles[0].rotate(angle as f32, &center);
        // renderer_todo.mesh = mesh;

        let mut viewpoint = &mut renderer_todo.view_point;

        for event in _events {
            if let Event::Key(key_event) = event {
                if let KeyCode::Char(char) = key_event.code {
                    match char {
                        'a' => {
                            viewpoint.x -= 1.0;
                        },
                        'd' => {
                            viewpoint.x += 1.0;
                        },
                        's' => {
                            viewpoint.z -= 1.0;
                        },
                        'w' => {
                            viewpoint.z += 1.0;
                        },
                        'r' => {
                            viewpoint.y -= 1.0;
                        },
                        'f' => {
                            viewpoint.y += 1.0;
                        },
                        _ => (),
                    }
                }
            }
        }

        ShouldExit::No
    };

    renderer.start_rendering(&frame_loop);
}
