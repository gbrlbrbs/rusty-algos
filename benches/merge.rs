use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rusty_algos::sorting::merge_sort;
use rand::{distributions::Uniform, Rng};

fn merge_5(c: &mut Criterion) {
    let rng = rand::thread_rng();
    let distr = Uniform::new(-1_000_000, 1_000_000);
    let samples: Vec<i32> = rng.sample_iter(&distr).take(100_000).collect();
    c.bench_function(
        "merge 10^5",
        |b| b.iter(|| merge_sort(black_box(&samples)))
    );
}

fn merge_6(c: &mut Criterion) {
    let rng = rand::thread_rng();
    let distr = Uniform::new(-1_000_000, 1_000_000);
    let samples: Vec<i32> = rng.sample_iter(&distr).take(1_000_000).collect();
    c.bench_function(
        "merge 10^6",
        |b| b.iter(|| merge_sort(black_box(&samples)))
    );
}

criterion_group!(benches, merge_5, merge_6);
criterion_main!(benches);