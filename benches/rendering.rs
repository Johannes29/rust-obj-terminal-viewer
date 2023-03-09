use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, Criterion};
use rust_obj_terminal_viewer::renderer::{obj_parser::ObjParser, interface::Renderer};

// to use: run `cargo bench --bench rendering` in terminal
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("some-bench-group");
    group.significance_level(0.1).sample_size(30);

    let mut renderer = Renderer::new(
        120,
        60,
        60.0,
        2.0,
        70.0,
        " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$",
    );
    let mesh = ObjParser::parse_file(&PathBuf::from("objects/Tree1.obj")).unwrap();
    renderer.set_mesh(mesh);
    group.bench_function("rendering", |b| b.iter(|| {
        renderer.render_frame();
    }));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);