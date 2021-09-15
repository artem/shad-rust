use bst::BstSet;

use pretty_assertions::assert_eq;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

use std::collections::HashSet;

#[test]
fn test_random_insertions_small() {
    let mut rng = StdRng::seed_from_u64(23254452323);
    for _ in 0..1000 {
        let mut bst_set = BstSet::new();
        let mut hash_set = HashSet::new();

        for _ in 0..10 {
            assert_eq!(bst_set.len(), hash_set.len());
            for x in -10..10 {
                assert_eq!(bst_set.contains(x), hash_set.contains(&x));
            }

            let y = rng.gen_range(-10..10);
            assert_eq!(bst_set.insert(y), hash_set.insert(y));
        }
    }
}

#[test]
fn test_random_insertions_big() {
    let mut rng = StdRng::seed_from_u64(77254452323);
    for _ in 0..100 {
        let mut bst_set = BstSet::new();
        let mut hash_set = HashSet::new();

        for _ in 0..100 {
            assert_eq!(bst_set.len(), hash_set.len());
            for _ in 0..100 {
                let x = rng.gen::<i64>();
                assert_eq!(bst_set.contains(x), hash_set.contains(&x));
            }

            for &x in hash_set.iter() {
                assert_eq!(bst_set.contains(x), hash_set.contains(&x));
            }

            let x = rng.gen::<i64>();
            assert_eq!(bst_set.insert(x), hash_set.insert(x));
        }
    }
}

#[test]
fn test_random_removals_small() {
    let mut rng = StdRng::seed_from_u64(34572306520);
    for _ in 0..1000 {
        let mut bst_set = BstSet::new();
        let mut hash_set = HashSet::new();

        for _ in 0..10 {
            let x = rng.gen_range(-10..10);
            assert_eq!(bst_set.insert(x), hash_set.insert(x));
        }

        for _ in 0..20 {
            let x = rng.gen_range(-10..10);
            assert_eq!(bst_set.remove(x), hash_set.remove(&x));
            assert_eq!(bst_set.len(), hash_set.len());

            for y in -10..10 {
                assert_eq!(bst_set.contains(y), hash_set.contains(&y));
            }
        }
    }
}

#[test]
fn test_random_removals_big() {
    let mut rng = StdRng::seed_from_u64(357620523560);
    for _ in 0..100 {
        let mut bst_set = BstSet::new();
        let mut hash_set = HashSet::new();

        for _ in 0..100 {
            let x = rng.gen::<i64>();
            assert_eq!(bst_set.insert(x), hash_set.insert(x));
        }

        let mut removals: Vec<i64> = hash_set.iter().cloned().collect();
        for _ in 0..removals.len() {
            removals.push(rng.gen::<i64>());
        }
        removals.shuffle(&mut rng);

        for x in removals.into_iter() {
            assert_eq!(bst_set.remove(x), hash_set.remove(&x));
            assert_eq!(bst_set.len(), hash_set.len());

            for &z in hash_set.iter() {
                assert!(bst_set.contains(z));
            }
        }
    }
}
