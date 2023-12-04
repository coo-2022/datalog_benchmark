use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn my_bench_func(c: &mut Criterion) {
    c.bench_function("my1", |b| b.iter(||{
        // 测试代码
    }));
}

criterion_group!(benches, my_bench_func);
criterion_main!(benches);