use arrayvec::ArrayVec;

use std::{
    mem::{size_of, size_of_val},
    rc::Rc,
};

#[test]
fn test_simple() {
    let mut v = ArrayVec::<i32, 2>::new();
    assert!(size_of_val(&v) == 2 * size_of::<i32>() + size_of::<usize>());

    assert_eq!(v.pop(), None);
    assert_eq!(v.push(1), Ok(()));
    assert_eq!(v.pop(), Some(1));
    assert_eq!(v.pop(), None);

    assert_eq!(v.push(10), Ok(()));
    assert_eq!(v.push(25), Ok(()));
    assert_eq!(v.push(45), Err(45));
    assert_eq!(v[0], 10);
    assert_eq!(v[1], 25);
    v[1] = 350;
    assert_eq!(v[1], 350);
    assert_eq!(v.pop(), Some(350));
    assert_eq!(v[0], 10);
    assert_eq!(v.pop(), Some(10));
    assert_eq!(v.pop(), None);
}

#[test]
#[should_panic]
fn test_out_of_bounds_panic() {
    let mut v = ArrayVec::<i32, 100>::new();
    v.push(50).ok();
    v[1];
}

#[test]
#[should_panic]
fn test_out_of_bounds_mut_panic() {
    let mut v = ArrayVec::<i32, 0>::new();
    v[0] = 34;
}

#[test]
fn test_drop() {
    let obj = Rc::new(50);

    let mut v = ArrayVec::<_, 10>::new();
    for _ in 0..v.capacity() {
        v.push(obj.clone()).ok();
    }

    assert_eq!(Rc::strong_count(&obj), 11);
    v.pop();
    assert_eq!(Rc::strong_count(&obj), 10);
    drop(v);
    assert_eq!(Rc::strong_count(&obj), 1);
}
