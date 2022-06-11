use criterion::{criterion_group, criterion_main, Criterion, black_box};
use mandelbrot::complex::*;
use mandelbrot::pixels::*;

fn benchmark_renders(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rendering");

    let bounds = black_box(parse_pair("1000x750", 'x').unwrap());
    let upper_left = black_box(parse_complex("-1.2,0.35").unwrap());
    let lower_right = black_box(parse_complex("-1,0.2").unwrap());
    let mut plain_pixels = black_box(vec![0; bounds.0 * bounds.1]);
    let mut rayon_pixels = black_box(vec![0; bounds.0 * bounds.1]);
    let mut crossbeam_pixels = black_box(vec![0; bounds.0 * bounds.1]);
    group.bench_function("Plain", |b| b.iter(|| render(&mut plain_pixels, bounds, upper_left, lower_right)));
    group.bench_function("Rayon", |b| b.iter(|| rayon_render(&mut rayon_pixels, bounds, upper_left, lower_right)));
    group.bench_function("Crossbeam", |b| b.iter(|| crossbeam_render(&mut crossbeam_pixels, bounds, upper_left, lower_right)));
    group.finish();
}

criterion_group!(benches, benchmark_renders);
criterion_main!(benches);
