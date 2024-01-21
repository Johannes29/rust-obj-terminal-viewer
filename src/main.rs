use clap::Parser;
use crossterm::terminal;
use rust_obj_terminal_viewer::renderer::camera_rotation::CameraInputHelper;
use rust_obj_terminal_viewer::renderer::interface::Renderer;
use rust_obj_terminal_viewer::renderer::obj_parser::ObjParser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

// +x is to the right, +y is up, +z is forwards
fn main() {
    let args = Cli::parse();;
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

    let mut camera_input_helper = CameraInputHelper::new(terminal_size.0, terminal_size.1);
    let mut frame_loop = |renderer: &mut Renderer, events| {
        camera_input_helper.update_terminal_dimensions(terminal::size().unwrap());
        camera_input_helper.process_input_events(renderer, events)
    };

    renderer.start_rendering(&mut frame_loop);
}
