use flatmap::FlatMap;

use pretty_assertions::assert_eq;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

use std::{collections::HashMap, iter::FromIterator};

#[test]
fn test_basics() {
    let mut map = FlatMap::new();
    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);

    assert_eq!(map.insert(1, 20), None);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&20));
    assert_eq!(map.get(&75), None);
    assert_eq!(map[&1], 20);
    assert_eq!(map.as_slice(), &[(1, 20)]);

    assert_eq!(map.insert(1, 50), Some(20));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&50));
    assert_eq!(map[&1], 50);
    assert_eq!(map.as_slice(), &[(1, 50)]);

    assert_eq!(map.remove(&1), Some(50));
    assert_eq!(map.len(), 0);
    assert_eq!(map.get(&1), None);
    assert_eq!(map.as_slice(), &[]);

    assert_eq!(map.remove(&1), None);

    assert_eq!(map.insert(30, 45), None);
    assert_eq!(map.remove_entry(&30), Some((30, 45)));
    assert_eq!(map.remove_entry(&10), None);

    let map = map;
    assert_eq!(map.len(), 0);
    assert!(map.capacity() > 0);
    assert_eq!(map.get(&100), None);
}

#[test]
fn test_str() {
    let mut map = FlatMap::new();

    map.insert("key".to_string(), 200);
    assert_eq!(map.get("key"), Some(&200));
    assert_eq!(map.get("nope"), None);
    assert_eq!(map["key"], 200);

    assert_eq!(map.insert("key".to_string(), 500), Some(200));
    assert_eq!(map.remove("key"), Some(500));
    assert_eq!(map.remove("key"), None);

    assert_eq!(map.insert("foo".to_string(), 123), None);
    assert_eq!(map.remove_entry("foo"), Some(("foo".to_string(), 123)));
    assert_eq!(map.remove_entry("foo"), None);
}

#[test]
fn test_conversions() {
    let map_one = FlatMap::from(vec![(3, 30), (2, 20), (1, 10)]);
    let map_two: FlatMap<_, _> = vec![(2, 20), (1, 10), (3, 30)].into();
    let map_three = FlatMap::from_iter({
        let mut map = HashMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        map
    });
    let expected = &[(1, 10), (2, 20), (3, 30)];
    assert_eq!(map_one.as_slice(), expected);
    assert_eq!(map_two.as_slice(), expected);
    assert_eq!(map_three.as_slice(), expected);

    let vec_one = Vec::from(map_one);
    let vec_two: Vec<_> = map_two.into();
    let vec_three = Vec::from_iter(map_three);
    assert_eq!(vec_one.as_slice(), expected);
    assert_eq!(vec_two.as_slice(), expected);
    assert_eq!(vec_three.as_slice(), expected);
}

#[test]
fn test_dedup() {
    let map_one = FlatMap::from(vec![(1, 1), (5, 5), (4, 41), (4, 40), (4, 44)]);
    let map_two = FlatMap::from_iter(vec![(5, 5), (4, 42), (4, 48), (4, 44), (1, 1)]);
    let map_three = {
        let mut map = FlatMap::new();
        map.extend(vec![(4, 325), (5, 5), (4, 46), (1, 1), (4, 30), (4, 44)]);
        map
    };

    let expected = &[(1, 1), (4, 44), (5, 5)];
    assert_eq!(map_one.as_slice(), expected);
    assert_eq!(map_two.as_slice(), expected);
    assert_eq!(map_three.as_slice(), expected);
}

#[test]
fn test_random_insertions_small() {
    let mut rng = StdRng::seed_from_u64(23254452323);
    for _ in 0..1000 {
        let mut flat_map = FlatMap::new();
        let mut hash_map = HashMap::new();

        for _ in 0..10 {
            assert_eq!(flat_map.len(), hash_map.len());
            for key in -10..10 {
                assert_eq!(flat_map.get(&key), hash_map.get(&key));
            }

            let key = rng.gen_range(-10..10);
            let value = rng.gen_range(-10..10);
            assert_eq!(flat_map.insert(key, value), hash_map.insert(key, value));
        }
    }
}

#[test]
fn test_random_insertions_big() {
    let mut rng = StdRng::seed_from_u64(77254452323);
    for _ in 0..100 {
        let mut flat_map = FlatMap::new();
        let mut hash_map = HashMap::new();

        for _ in 0..100 {
            assert_eq!(flat_map.len(), hash_map.len());
            for _ in 0..100 {
                let key = rng.gen::<i64>();
                assert_eq!(flat_map.get(&key), hash_map.get(&key));
            }

            for key in hash_map.keys() {
                assert_eq!(flat_map.get(key), hash_map.get(key));
            }

            let key = rng.gen::<i64>();
            let value = rng.gen::<i64>();
            assert_eq!(flat_map.insert(key, value), hash_map.insert(key, value));
        }
    }
}

#[test]
fn test_random_removals_small() {
    let mut rng = StdRng::seed_from_u64(34572306520);
    for _ in 0..1000 {
        let mut flat_map = FlatMap::new();
        let mut hash_map = HashMap::new();

        for _ in 0..10 {
            let key = rng.gen_range(-10..10);
            let value = rng.gen_range(-10..10);
            assert_eq!(flat_map.insert(key, value), hash_map.insert(key, value));
        }

        for _ in 0..20 {
            let key = rng.gen_range(-10..10);
            assert_eq!(flat_map.remove(&key), hash_map.remove(&key));
            assert_eq!(flat_map.len(), hash_map.len());

            for key in -10..10 {
                assert_eq!(flat_map.get(&key), hash_map.get(&key));
            }
        }
    }
}

#[test]
fn test_random_removals_big() {
    let mut rng = StdRng::seed_from_u64(357620523560);
    for _ in 0..100 {
        let mut flat_map = FlatMap::new();
        let mut hash_map = HashMap::new();

        for _ in 0..100 {
            let key = rng.gen::<i64>();
            let value = rng.gen::<i64>();
            assert_eq!(flat_map.insert(key, value), hash_map.insert(key, value));
        }

        let mut removals: Vec<i64> = hash_map.keys().cloned().collect();
        for _ in 0..removals.len() {
            removals.push(rng.gen::<i64>());
        }
        removals.shuffle(&mut rng);

        for key in removals.into_iter() {
            assert_eq!(flat_map.remove(&key), hash_map.remove(&key));
            assert_eq!(flat_map.len(), hash_map.len());

            for key in hash_map.keys() {
                assert_eq!(flat_map.get(key), hash_map.get(key));
            }
        }
    }
}
