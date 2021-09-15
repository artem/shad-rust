#![forbid(unsafe_code)]

use std::cmp::Ordering;

struct Node {
    key: i64,
    left_ptr: Option<Box<Node>>,
    right_ptr: Option<Box<Node>>,
}

#[derive(Default)]
pub struct BstSet {
    // TODO: your code here.
}

impl BstSet {
    pub fn new() -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn is_empty(&self) -> bool {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn contains(&self, key: i64) -> bool {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn insert(&mut self, key: i64) -> bool {
        // If the key is already contained in set, return false.
        // Otherwise insert the key and return true.
        // TODO: your code here.
        unimplemented!()
    }

    pub fn remove(&mut self, key: i64) -> bool {
        // If the key is contained in set, remove it and return true.
        // Otherwise return false.
        // TODO: your code here.
        unimplemented!()
    }

}
