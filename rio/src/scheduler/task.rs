use futures::FutureExt;

use std::{
    fmt::Display,
    future::Future,
    ops::Deref,
    pin::Pin,
    task::{Context, Poll},
};

////////////////////////////////////////////////////////////////////////////////

pub struct Task(Pin<Box<dyn Future<Output = ()> + Send>>);

impl Task {
    pub fn id(&self) -> TaskId {
        TaskId(self.0.deref() as *const dyn Future<Output = ()> as *const () as usize)
    }

    pub fn poll(&mut self, cx: &mut Context) -> Poll<()> {
        self.0.as_mut().poll_unpin(cx)
    }
}

impl<T> From<T> for Task
where
    T: Future<Output = ()> + Send + 'static,
{
    fn from(value: T) -> Self {
        Self(value.boxed())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TaskId(usize);

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}
