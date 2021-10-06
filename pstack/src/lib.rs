#![forbid(unsafe_code)]

use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////

pub struct PRef<T> {
    // TODO: your code here.
}

impl<T> std::ops::Deref for PRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // TODO: your code here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct PStack<T> {
    // TODO: your code here.
}

impl<T> Default for PStack<T> {
    fn default() -> Self {
        // TODO: your code here.
        unimplemented!()
    }
}

impl<T> Clone for PStack<T> {
    fn clone(&self) -> Self {
        // TODO: your code here.
        unimplemented!()
    }
}

impl<T> PStack<T> {
    pub fn new() -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn push(&self, value: T) -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
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

    pub fn iter(&self) -> impl Iterator<Item = PRef<T>> {
        // TODO: your code here.
        unimplemented!()
    }
}

