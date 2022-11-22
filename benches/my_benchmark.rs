use std::time::{Instant, Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_obj_terminal_viewer::renderer::interface::{Renderer, ShouldExit};
use rust_obj_terminal_viewer::renderer::obj_parser::parse_obj;


use crossterm::event::Event;
use crossterm::terminal;

pub fn criterion_benchmark(c: &mut Criterion) {
    let terminal_size = terminal::size().unwrap();
    let mut renderer = Renderer::new(terminal_size.0, terminal_size.1, 10.0, 2.0, 90.0, " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$");
    let mesh = parse_obj("objects/cube_inverted_top.obj");
    renderer.mesh = mesh;
    
    let test_duration = Duration::new(10, 0);
    let start_time = Instant::now();

    let frame_loop = |renderer_todo: &mut Renderer, _events: Vec<Event>| -> ShouldExit {
        if start_time.elapsed() > test_duration {

        }
        ShouldExit::No
    };

    c.bench_function("fib 20", |b| b.iter(|| renderer.start_rendering(&frame_loop)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
