extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use saku::SentenceTokenizer;
use std::{fs, time::Duration};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("raw-tokenize", |b| {
        let data_path = "../data/medium.txt";
        let tokenizer = SentenceTokenizer::default();
        let text = fs::read_to_string(data_path).unwrap();
        b.iter(|| tokenizer.tokenize_raw(&text))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10).warm_up_time(Duration::from_secs(10)).with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}
criterion_main!(benches);
