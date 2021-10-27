use crate::{
    data::ObjectId,
    error::*,
    object::{
        Object,
    },
    storage::StorageTransaction,
};

use std::{
    any::{Any, TypeId},
    cell::{Cell, Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    marker::PhantomData,
    rc::Rc,
};

////////////////////////////////////////////////////////////////////////////////

pub struct Transaction<'a> {
    inner: Box<dyn StorageTransaction + 'a>,
    // TODO: your code here.
}

impl<'a> Transaction<'a> {
    pub(crate) fn new(inner: Box<dyn StorageTransaction + 'a>) -> Self {
        Self {
            inner,
            // TODO: your code here.
        }
    }

    pub fn create<T: Object>(&self, obj: T) -> Result<Tx<'_, T>> {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn get<T: Object>(&self, id: ObjectId) -> Result<Tx<'_, T>> {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn commit(self) -> Result<()> {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn rollback(self) -> Result<()> {
        // TODO: your code here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub enum ObjectState {
    Clean,
    Modified,
    Removed,
}

#[derive(Clone)]
pub struct Tx<'a, T> {
    // TODO: your code here.
    lifetime: PhantomData<&'a T>,
}

impl<'a, T: Any> Tx<'a, T> {
    pub fn id(&self) -> ObjectId {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn state(&self) -> ObjectState {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn delete(self) {
        // TODO: your code here.
        unimplemented!()
    }
}

