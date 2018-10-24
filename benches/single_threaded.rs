use std::time::{Duration, Instant};

use super::variants::{BenchmarkDirectBucket, BenchmarkKeyedBucket, Variant};

use criterion::{black_box, Criterion, ParameterizedBenchmark, Throughput};

pub fn bench_all(c: &mut Criterion) {
    bench_direct(c);
    bench_keyed(c);
}

fn bench_direct(c: &mut Criterion) {
    let id = "single_threaded/direct";
    let bm = ParameterizedBenchmark::new(
        id,
        move |b, ref v| {
            run_with_variants!(v, rl: BenchmarkDirectBucket, {
                let now = Instant::now();
                let ms = Duration::from_millis(20);
                let mut i = 0;
                b.iter(|| {
                    i += 1;
                    black_box(rl.check_at(now + (ms * i)).is_ok());
                });
            });
        },
        Variant::ALL,
    ).throughput(|_s| Throughput::Elements(1));
    c.bench(id, bm);
}

fn bench_keyed(c: &mut Criterion) {
    let id = "single_threaded/keyed";
    let bm = ParameterizedBenchmark::new(
        id,
        move |b, ref v| {
            run_with_variants!(v, rl: BenchmarkKeyedBucket, {
                let now = Instant::now();
                let ms = Duration::from_millis(20);
                let mut i = 0;
                b.iter(|| {
                    i += 1;
                    black_box(rl.check_at(i % 100, now + (ms * i)).is_ok());
                });
            });
        },
        Variant::ALL,
    ).throughput(|_s| Throughput::Elements(1));
    c.bench(id, bm);
}
