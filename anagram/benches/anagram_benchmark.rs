use anagram::anagram_string;
use criterion::{criterion_group, criterion_main, Criterion};

fn is_anagram_benchmark(c: &mut Criterion) {
    c.bench_function("is_anagram", |b| {
        b.iter(|| {
            // Call your function with the inputs to benchmark
            let _result = anagram_string("hamza".to_string(), "azmah".to_string());
        });
    });
}

criterion_group!(benches, is_anagram_benchmark);
criterion_main!(benches);