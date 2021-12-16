#![no_std]

use core::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

pub struct ArrayVec<T, const N: usize> {
    // TODO: your code here.
}

impl<T, const N: usize> ArrayVec<T, N> {
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

    pub fn push(&mut self, obj: T) -> Option<T> {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn pop(&mut self) -> Option<T> {
        // TODO: your code here.
        unimplemented!()
    }
}

impl<T, const N: usize> Default for ArrayVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Index<usize> for ArrayVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        // TODO: your code here.
        unimplemented!()
    }
}

impl<T, const N: usize> IndexMut<usize> for ArrayVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // TODO: your code here.
        unimplemented!()
    }
}
