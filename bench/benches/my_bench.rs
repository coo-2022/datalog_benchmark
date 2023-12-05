use criterion::{black_box, criterion_group, criterion_main, Criterion};
extern crate bench;
use bench::{rust, datalog};

fn rust_bench(c: &mut Criterion) {
    c.bench_function("rust_bench", |b| b.iter(||{
        let mut table1 = Vec::new();
        let mut table2 = Vec::new();
        for i in 1..=1000 {
            table1.push((i as i32, i * 10 as u64));
        }
        for i in 500..=1499 {
            table2.push((i as i32, i * 100 as u64));
        }
        rust::join_tables(table1, table2)
    }));
}

fn datalog_bench(c: &mut Criterion) {
    c.bench_function("datalog_bench", |b| b.iter(||{
        let mut table1 = Vec::new();
        let mut table2 = Vec::new();
        for i in 1..=1000 {
            table1.push((i as i32, i * 10 as u64));
        }
        for i in 500..=1499 {
            table2.push((i as i32, i * 100 as u64));
        }
        datalog::join_tables(table1, table2)
    }));
}

criterion_group!(benches, rust_bench, datalog_bench);
criterion_main!(benches);