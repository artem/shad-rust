use ::std::{
    collections::{HashMap as __HashMap, VecDeque as __VecDeque},
    vec as __vec,
};

use stdmacro::{deque, map, sorted_vec};

#[allow(unused)]
macro_rules! vec {
    () => {};
}

#[allow(unused)]
mod std {
    mod collections {
        pub struct HashMap;
        pub struct Vec;
        pub struct VecDeque;
    }
}

pub struct Wrapper(usize);

#[derive(PartialEq, Eq, Hash)]
pub struct Hashable(usize);

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub struct Comparable(usize);

#[test]
fn test_deque() {
    let mut d = deque![1, 2, 3];
    assert_eq!(d.pop_front(), Some(1));
    assert_eq!(d.pop_back(), Some(3));
    assert_eq!(d.pop_front(), Some(2));
    assert_eq!(d.pop_back(), None);
    assert_eq!(d.pop_front(), None);

    let mut d2 = deque![8; 10];
    for _ in 0..10 {
        assert_eq!(d2.pop_back(), Some(8));
    }

    let d3: __VecDeque<i32> = deque![];
    assert_eq!(d3, __VecDeque::new());

    let mut d4 = deque![Wrapper(0), Wrapper(10), Wrapper(135)];
    assert_eq!(d4.pop_back().unwrap().0, 135);
    assert_eq!(d4.pop_front().unwrap().0, 0);
    assert_eq!(d4.pop_front().unwrap().0, 10);
}

#[test]
fn test_sorted_vec() {
    let v = sorted_vec![4, 3, 2, 1, 5, 2, 3, 4];
    assert_eq!(v, __vec![1, 2, 2, 3, 3, 4, 4, 5]);

    let empty: Vec<i32> = sorted_vec![];
    assert_eq!(empty, Vec::new());

    let v2 = sorted_vec![Comparable(5), Comparable(10), Comparable(7)];
    assert!(v2 == __vec![Comparable(5), Comparable(7), Comparable(10)]);
}

#[test]
fn test_map() {
    let m = map! {
        "foo" => 10,
        "bar" => 20,
    };
    assert_eq!(m["foo"], 10);
    assert_eq!(m["bar"], 20);

    let m2 = map! {
        Hashable(220) => Wrapper(30)
    };
    assert_eq!(m2[&Hashable(220)].0, 30);

    let m3: __HashMap<String, i32> = map! {};
    assert_eq!(__HashMap::<String, i32>::new(), m3);
}
