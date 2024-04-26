use clap::Parser;
use crossterm::terminal;
use rust_obj_terminal_viewer::general::positions_3d::{BoundingBox, Point as Point3};
use rust_obj_terminal_viewer::renderer::camera_rotation::CameraInputHelper;
use rust_obj_terminal_viewer::renderer::interface::Renderer;
use rust_obj_terminal_viewer::renderer::obj_parser::ObjParser;

const SHORT_ABOUT_TEXT: &str =
    "A CLI program to view 3D models directly in the terminal. Supports .obj files.";

#[derive(Parser)]
#[command(
    about = SHORT_ABOUT_TEXT,
    long_about = format!("{SHORT_ABOUT_TEXT}\n\n\
    To rotate the object, move the mouse with the left mouse button pressed. \
    To close the program, press `q`.")
)]
struct Cli {
    path: std::path::PathBuf,
}

// +x is to the right, +y is up, -z is forwards
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
    renderer.info_text = Some("press q to exit".to_owned());

    let radius =
        BoundingBox::new(&renderer.mesh.points).get_longest_distance_from_point(&Point3::new());
    let camera_distance = renderer.camera.distance_to_fit_sphere(radius);
    let mut camera_input_helper =
        CameraInputHelper::new(terminal_size.0, terminal_size.1, camera_distance);
    renderer.camera.far = camera_distance + radius;
    renderer.camera.near = camera_distance - radius;

    let mut frame_loop = |renderer: &mut Renderer, events| {
        camera_input_helper.process_input_events(events);
        camera_input_helper.apply_to_camera(&mut renderer.camera);
    };

    renderer.start_rendering(&mut frame_loop);
}
