use chrono::NaiveDate;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use timespent::graph::x_segments::ScaleXSegments;

pub fn criterion_benchmark(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd_opt(2015, 3, 14).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2030, 3, 14).unwrap();
    c.bench_function("ScaleXSegments.new", |b| {
        b.iter(|| ScaleXSegments::new(black_box(&start_date), black_box(&end_date)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
