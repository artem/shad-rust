use bst::BstSet;
use cbst::CBstSet;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

fn bench_100k_random_insertions(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(325261642346);
    let insertions: Vec<i64> = (0..100_000).map(|_| rng.gen::<i64>()).collect();

    let mut group = c.benchmark_group("100k_random_insertions");

    group.bench_function("rust_BstSet", |b| {
        b.iter(|| {
            black_box({
                let mut set = BstSet::new();
                for &x in insertions.iter() {
                    set.insert(x);
                }
                set
            })
        })
    });

    group.bench_function("c_BstSet", |b| {
        b.iter(|| {
            black_box({
                let mut set = CBstSet::new();
                for &x in insertions.iter() {
                    set.insert(x);
                }
                set
            })
        })
    });
}

fn bench_100k_random_lookup_hits(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(23254452323);
    let mut insertions: Vec<i64> = (0..100_000).map(|_| rng.gen::<i64>()).collect();

    let mut bst_set = BstSet::new();
    for &x in insertions.iter() {
        bst_set.insert(x);
    }

    let mut cbst_set = CBstSet::new();
    for &x in insertions.iter() {
        cbst_set.insert(x);
    }

    insertions.shuffle(&mut rng);

    let mut group = c.benchmark_group("100k_random_lookup_hits");

    group.bench_function("rust_BstSet", |b| {
        b.iter(|| {
            black_box({
                let mut r = false;
                for &x in insertions.iter() {
                    r ^= bst_set.contains(x);
                }
                r
            })
        })
    });

    group.bench_function("c_BstSet", |b| {
        b.iter(|| {
            black_box({
                let mut r = false;
                for &x in insertions.iter() {
                    r ^= cbst_set.contains(x);
                }
                r
            })
        })
    });
}

fn bench_100k_random_lookup_misses(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(5430426233246);

    let mut bst_set = BstSet::new();
    let mut cbst_set = CBstSet::new();
    for _ in 0..100_000 {
        let val = rng.gen::<i64>() & !1;
        bst_set.insert(val);
        cbst_set.insert(val);
    }

    let lookups: Vec<i64> = (0..100_000).map(|_| rng.gen::<i64>() | 1).collect();

    let mut group = c.benchmark_group("100k_random_lookup_misses");

    group.bench_function("rust_BstSet", |b| {
        b.iter(|| {
            black_box({
                let mut r = false;
                for &x in lookups.iter() {
                    r ^= bst_set.contains(x);
                }
                r
            })
        })
    });

    group.bench_function("c_BstSet", |b| {
        b.iter(|| {
            black_box({
                let mut r = false;
                for &x in lookups.iter() {
                    r ^= cbst_set.contains(x);
                }
                r
            })
        })
    });
}

criterion_group!(
    benches,
    bench_100k_random_insertions,
    bench_100k_random_lookup_hits,
    bench_100k_random_lookup_misses,
);

criterion_main!(benches);
