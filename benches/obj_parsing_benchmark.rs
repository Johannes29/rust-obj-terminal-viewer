use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_obj_terminal_viewer::renderer::obj_parser::ObjParser;

// to use: run `cargo bench` in terminal
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-10");
    group.significance_level(0.1).sample_size(10);

    let obj_path = PathBuf::from("objects/Tree1.obj");
    group.bench_function("obj parser", |b| b.iter(|| {
        match ObjParser::parse_file(black_box(&obj_path)) {
            Ok(_) => (),
            Err(message) => println!("{}", message),
        };
    }));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);