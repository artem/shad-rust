use flatmap::FlatMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

use std::{
    collections::{hash_map::RandomState, BTreeMap, HashMap},
    iter::FromIterator,
};

fn bench_100k_random_lookup_hits(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(23254452323);
    let mut insertions: Vec<_> = (0..100_000)
        .map(|_| (rng.gen::<i64>(), rng.gen::<i64>()))
        .collect();

    let flat_map = FlatMap::from_iter(insertions.iter().copied());
    let btree_map = BTreeMap::from_iter(insertions.iter().copied());
    let hash_map = HashMap::<_, _, RandomState>::from_iter(insertions.iter().copied());

    insertions.shuffle(&mut rng);

    let mut group = c.benchmark_group("100k_random_lookup_hits");

    group.bench_function("flat_map", |b| {
        b.iter(|| {
            black_box({
                let mut r = 0;
                for &(key, _) in insertions.iter() {
                    r ^= flat_map.get(&key).unwrap();
                }
                r
            })
        })
    });

    group.bench_function("btree_map", |b| {
        b.iter(|| {
            black_box({
                let mut r = 0;
                for &(key, _) in insertions.iter() {
                    r ^= btree_map.get(&key).unwrap();
                }
                r
            })
        })
    });

    group.bench_function("hash_map", |b| {
        b.iter(|| {
            black_box({
                let mut r = 0;
                for &(key, _) in insertions.iter() {
                    r ^= hash_map.get(&key).unwrap();
                }
                r
            })
        })
    });
}

fn bench_100k_random_lookup_misses(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(5430426233246);

    let mut flat_map = FlatMap::new();
    let mut btree_map = BTreeMap::new();
    let mut hash_map = HashMap::new();
    for _ in 0..100_000 {
        let key = rng.gen::<i64>() & !1;
        let value = rng.gen::<i64>();
        flat_map.insert(key, value);
        btree_map.insert(key, value);
        hash_map.insert(key, value);
    }

    let lookups: Vec<i64> = (0..100_000).map(|_| rng.gen::<i64>() | 1).collect();

    let mut group = c.benchmark_group("100k_random_lookup_misses");

    group.bench_function("flat_map", |b| {
        b.iter(|| {
            black_box({
                let mut r = 0;
                for key in lookups.iter() {
                    r ^= flat_map.get(key).unwrap_or(&1);
                }
                r
            })
        })
    });

    group.bench_function("btree_map", |b| {
        b.iter(|| {
            black_box({
                let mut r = 0;
                for key in lookups.iter() {
                    r ^= btree_map.get(key).unwrap_or(&1);
                }
                r
            })
        })
    });

    group.bench_function("hash_map", |b| {
        b.iter(|| {
            black_box({
                let mut r = 0;
                for key in lookups.iter() {
                    r ^= hash_map.get(key).unwrap_or(&1);
                }
                r
            })
        })
    });
}

criterion_group!(
    benches,
    bench_100k_random_lookup_hits,
    bench_100k_random_lookup_misses,
);

criterion_main!(benches);
