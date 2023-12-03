use crate::{
    network::{NetworkDriver, NetworkHandle},
    scheduler::{Scheduler, TaskId},
    timer::{TimerDriver, TimerHandle},
};

use futures::{channel::oneshot, Future, FutureExt};
use thiserror::Error;

use std::{
    cell::RefCell,
    pin::Pin,
    sync::{Arc, Weak},
    task::{Context, Poll},
};

////////////////////////////////////////////////////////////////////////////////

thread_local! {
    static RUNTIME_HANDLE: RefCell<Option<RuntimeHandle>> = RefCell::new(None);
}

pub fn spawn<T>(future: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send,
{
    RuntimeHandle::current().spawn(future)
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Runtime(Arc<RuntimeState>);

impl Runtime {
    pub fn handle(&self) -> RuntimeHandle {
        RuntimeHandle(Arc::downgrade(&self.0))
    }

    pub fn spawn<T>(&self, future: T) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send,
    {
        self.0.spawn(future)
    }

    pub fn block_on<T>(&self, future: T) -> T::Output
    where
        T: Future + Send + 'static,
        T::Output: Send,
    {
        let _guard = ContextGuard::new(self.handle());
        let (task_id, mut handle) = self.0.submit(future);
        self.0.scheduler.block_on(task_id);
        match handle.receiver.try_recv() {
            Ok(Some(value)) => value,
            _ => unreachable!(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct RuntimeHandle(Weak<RuntimeState>);

impl RuntimeHandle {
    pub fn current() -> Self {
        RUNTIME_HANDLE.with(|h| {
            h.borrow()
                .as_ref()
                .expect("this function must be called from within a runtime")
                .clone()
        })
    }

    pub fn spawn<T>(&self, future: T) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send,
    {
        self.state().spawn(future)
    }

    pub(crate) fn state(&self) -> Arc<RuntimeState> {
        self.0.upgrade().expect("the runtime has been dropped")
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct RuntimeState {
    pub scheduler: Scheduler,
    pub timer_handle: TimerHandle,
    pub network_handle: NetworkHandle,
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self {
            scheduler: Scheduler::default(),
            timer_handle: TimerDriver::start(),
            network_handle: NetworkDriver::start(),
        }
    }
}

impl RuntimeState {
    pub fn spawn<T>(&self, future: T) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send,
    {
        self.submit(future).1
    }

    fn submit<T>(&self, future: T) -> (TaskId, JoinHandle<T::Output>)
    where
        T: Future + Send + 'static,
        T::Output: Send,
    {
        let (sender, receiver) = oneshot::channel();
        let task = async move {
            let _ = sender.send(future.await);
        };
        let task_id = self.scheduler.submit(task);
        let handle = JoinHandle { receiver };
        (task_id, handle)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct JoinHandle<T> {
    receiver: oneshot::Receiver<T>,
}

#[derive(Debug, Error)]
#[error("the task has been dropped")]
pub struct JoinError {}

impl<T> Future for JoinHandle<T> {
    type Output = Result<T, JoinError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut()
            .receiver
            .poll_unpin(cx)
            .map_err(|_| JoinError {})
    }
}

////////////////////////////////////////////////////////////////////////////////

struct ContextGuard {}

impl ContextGuard {
    fn new(handle: RuntimeHandle) -> Self {
        RUNTIME_HANDLE.with(|h| {
            *h.borrow_mut() = Some(handle);
        });
        Self {}
    }
}

impl Drop for ContextGuard {
    fn drop(&mut self) {
        RUNTIME_HANDLE.with(|h| {
            *h.borrow_mut() = None;
        });
    }
}
