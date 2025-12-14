use criterion::{black_box, criterion_group, criterion_main, Criterion};
use allpass_filter::{AllPassFilter, Linear, Cubic};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Interpolation Comparison");

    group.bench_function("Linear Interpolation", |b| {
        let mut allpass_filter = AllPassFilter::new(1000, 10.5, 0.5, Linear);
        b.iter(|| {
            allpass_filter.process(black_box(1.0));
        })
    });

    group.bench_function("Cubic Interpolation", |b| {
        let mut allpass_filter = AllPassFilter::new(1000, 10.5, 0.5, Cubic);
        b.iter(|| {
            allpass_filter.process(black_box(1.0));
        })
    });

    group.finish();

    let mut group_block = c.benchmark_group("Block Processing Comparison");

    const BUFFER_SIZE: usize = 256;
    let input = [0.0f32; BUFFER_SIZE];
    let mut output = [0.0f32; BUFFER_SIZE];

    group_block.bench_function("Process Block with Linear Interpolation", |b| {
        let mut allpass_filter = AllPassFilter::new_default(1000, 10.5, 0.5);
        b.iter(|| {
            allpass_filter.process_block(black_box(&input), black_box(&mut output));
        })
    });

    group_block.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);