use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use timespent::{graph::ui::Graph, loader};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Load.from_gen", |b| {
        b.iter(|| {
            let activities = loader::load_from_filepath("../fake").expect("Failed to load data");
            Graph::new(&activities);
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().
        with_profiler(PProfProfiler::new(100, Output::Flamegraph(None))).
        measurement_time(std::time::Duration::from_secs(15));
    targets = criterion_benchmark
}
criterion_main!(benches);
