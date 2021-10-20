#![forbid(unsafe_code)]

use std::{
    cell::RefCell,
    collections::VecDeque,
    iter::{from_fn, repeat_with},
    rc::Rc,
};

pub fn count() -> impl Iterator<Item = u64> {
    // TODO: your code here.
    unimplemented!()
}

pub fn cycle<T>(into_iter: T) -> impl Iterator<Item = T::Item>
where
    T: IntoIterator,
    T::Item: Clone,
{
    // TODO: your code here.
    unimplemented!()
}

pub fn extract<T: IntoIterator>(
    into_iter: T,
    index: usize,
) -> (Option<T::Item>, impl Iterator<Item = T::Item>) {
    // TODO: your code here.
    unimplemented!()
}

pub fn tee<T>(into_iter: T) -> (impl Iterator<Item = T::Item>, impl Iterator<Item = T::Item>)
where
    T: IntoIterator,
    T::Item: Clone,
{
    // TODO: your code here.
    unimplemented!()
}

pub fn group_by<T, F, V>(into_iter: T, mut f: F) -> impl Iterator<Item = (V, Vec<T::Item>)>
where
    T: IntoIterator,
    F: FnMut(&T::Item) -> V,
    V: Eq,
{
    // TODO: your code here.
    unimplemented!()
}
