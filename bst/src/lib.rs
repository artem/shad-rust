#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::mem::swap;

struct Node {
    key: i64,
    left_ptr: Option<Box<Node>>,
    right_ptr: Option<Box<Node>>,
}

#[derive(Default)]
pub struct BstSet {
    root_ptr: Option<Box<Node>>,
    len: usize,
}

impl BstSet {
    pub fn new() -> Self {
        Self {
            root_ptr: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn contains(&self, key: i64) -> bool {
        let mut cur = &self.root_ptr;

        while let Some(node) = cur {
            match node.key.cmp(&key) {
                Ordering::Equal => return true,
                Ordering::Less => cur = &node.left_ptr,
                Ordering::Greater => cur = &node.right_ptr,
            }
        }

        false
    }

    pub fn insert(&mut self, key: i64) -> bool {
        // If the key is already contained in set, return false.
        // Otherwise insert the key and return true.
        let point = self.find_insertion_point(key);

        match point {
            None => {
                *point = Some(Box::new(Node {
                    key,
                    left_ptr: None,
                    right_ptr: None,
                }));
                self.len += 1;
                true
            }
            Some(_) => false,
        }
    }

    pub fn remove(&mut self, key: i64) -> bool {
        // If the key is contained in set, remove it and return true.
        // Otherwise return false.
        let mut point = self.find_insertion_point(key);

        if point.is_none() {
            return false;
        }

        while point.as_ref().unwrap().right_ptr.is_some() {
            point = Self::rotate_left(point);
        }
        *point = point.as_mut().unwrap().left_ptr.take();
        self.len -= 1;
        true
    }

    fn find_insertion_point(&mut self, key: i64) -> &mut Option<Box<Node>> {
        let mut cur = &mut self.root_ptr;

        while let Some(node) = &*cur {
            match node.key.cmp(&key) {
                Ordering::Equal => return cur,
                Ordering::Less => cur = &mut cur.as_mut().unwrap().left_ptr, // wtf
                Ordering::Greater => cur = &mut cur.as_mut().unwrap().right_ptr, // wtf
            }
        }

        cur
    }

    fn rotate_left(root: &mut Option<Box<Node>>) -> &mut Option<Box<Node>> {
        let mut a = root.take().unwrap();
        let mut c = a.right_ptr.unwrap();
        a.right_ptr = c.left_ptr;
        c.left_ptr = Some(a);
        *root = Some(c);
        &mut root.as_mut().unwrap().left_ptr
    }
}
