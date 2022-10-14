mod general;
mod renderer;
use crossterm::event::KeyCode;

use renderer::interface::{Renderer, ShouldExit};
use renderer::obj_parser::parse_obj;

use crossterm::event::Event;
use crossterm::terminal;

// +x is to the right, +y is up, +z is forwards
fn main() {
    let terminal_size = terminal::size().unwrap();
    let mut renderer = Renderer::new(terminal_size.0, terminal_size.1, 10.0, 2.0, 90.0, " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$");
    let mesh = parse_obj("objects/cube.obj");
    dbg!(&mesh);
    renderer.mesh = mesh;

    let frame_loop = |renderer_todo: &mut Renderer, _events: Vec<Event>| -> ShouldExit {

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
                        'f' => {
                            viewpoint.y -= 1.0;
                        },
                        'r' => {
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
