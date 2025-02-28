use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fluids;

fn criterion_benchmark(c: &mut Criterion) {
    let mut con = fluids::Container::new_random();
    c.bench_function("sim", |b| b.iter(|| con.run(black_box(1.))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
