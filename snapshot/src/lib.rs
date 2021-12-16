use thiserror::Error;

use std::{
    collections::HashMap,
    iter::{ExactSizeIterator, Iterator},
    marker::PhantomData,
    ops::Deref,
};

////////////////////////////////////////////////////////////////////////////////

pub type ObjectId = i64;

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
pub struct ResourceTotals {
    pub cpu: u64,
    pub memory: u64,
    pub disk_capacity: u64,
}

impl std::ops::AddAssign for ResourceTotals {
    fn add_assign(&mut self, other: Self) {
        self.cpu += other.cpu;
        self.memory += other.memory;
        self.disk_capacity += other.disk_capacity;
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Dump {
    pub node_segments: Vec<NodeSegmentRecord>,
    pub nodes: Vec<NodeRecord>,
    pub pod_sets: Vec<PodSetRecord>,
    pub pods: Vec<PodRecord>,
}

pub struct NodeSegmentRecord {
    pub id: ObjectId,
}

pub struct NodeRecord {
    pub id: ObjectId,
    pub node_segment_id: ObjectId,
    pub resource_totals: ResourceTotals,
}

pub struct PodSetRecord {
    pub id: ObjectId,
    pub node_segment_id: ObjectId,
}

pub struct PodRecord {
    pub id: ObjectId,
    pub pod_set_id: ObjectId,
    pub node_id: Option<ObjectId>,
    pub resource_requests: ResourceTotals,
}

////////////////////////////////////////////////////////////////////////////////

pub struct Sn<'a, T> {
    ptr: *const T,
    lifetime: PhantomData<&'a Snapshot>,
}

impl<'a, T> Deref for Sn<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // TODO: your code here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct NodeSegment {
    pub id: ObjectId,
    pub resource_usage: ResourceTotals,
    pub resource_requests: ResourceTotals,
    pub resource_totals: ResourceTotals,
    // TODO: your code here.
}


////////////////////////////////////////////////////////////////////////////////

pub struct Node {
    pub id: ObjectId,
    pub resource_usage: ResourceTotals,
    pub resource_totals: ResourceTotals,
    // TODO: your code here.
}


////////////////////////////////////////////////////////////////////////////////

pub struct PodSet {
    pub id: ObjectId,
    pub resource_requests: ResourceTotals,
    // TODO: your code here.
}


////////////////////////////////////////////////////////////////////////////////

pub struct Pod {
    pub id: ObjectId,
    pub resource_requests: ResourceTotals,
    // TODO: your code here.
}


////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Snapshot {
    // TODO: your code here.
}

impl Snapshot {
    pub fn new(dump: &Dump) -> Result<Self> {
        // TODO: your code here.
        unimplemented!()
    }

    // TODO: your code here.
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
pub enum ObjectType {
    NodeSegment,
    Node,
    PodSet,
    Pod,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("snapshot references a non-existent object (type: {ty:?}, id: {id})")]
    MissingObject { ty: ObjectType, id: ObjectId },
    #[error("found duplicate object in snapshot (type: {ty:?}, id: {id})")]
    DuplicateObject { ty: ObjectType, id: ObjectId },
}

pub type Result<T> = std::result::Result<T, Error>;
