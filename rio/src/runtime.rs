use crate::{
    scheduler::{JoinHandle, Scheduler},
    timer::{TimerDriver, TimerHandle},
};

#[cfg(feature = "net")]
use crate::network::{NetworkDriver, NetworkHandle};

use std::{
    cell::RefCell,
    future::Future,
    sync::{Arc, Weak},
};

////////////////////////////////////////////////////////////////////////////////

thread_local! {
    static RUNTIME_HANDLE: RefCell<Option<RuntimeHandle>> = RefCell::new(None);
}

pub fn runtime_id() -> RuntimeId {
    RuntimeHandle::current().id()
}

pub fn spawn<T>(future: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send,
{
    RuntimeHandle::current().spawn(future)
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeId(usize);

////////////////////////////////////////////////////////////////////////////////

pub struct Runtime(Arc<RuntimeState>);

impl Runtime {
    pub fn new_current_thread() -> Self {
        Self::new_with_scheduler(Scheduler::new_current_thread)
    }

    #[cfg(feature = "rt-multi-thread")]
    pub fn new_multi_thread(num_threads: usize) -> Self {
        Self::new_with_scheduler(move |context_manager| {
            Scheduler::new_multi_thread(context_manager, num_threads)
        })
    }

    fn new_with_scheduler(create_scheduler: impl FnOnce(ContextManager) -> Scheduler) -> Self {
        Self(Arc::new_cyclic(|weak| {
            let handle = RuntimeHandle(weak.clone());
            let context_manager = ContextManager { handle };
            RuntimeState {
                scheduler: create_scheduler(context_manager),
                timer_handle: TimerDriver::start(),

                #[cfg(feature = "net")]
                network_handle: NetworkDriver::start(),
            }
        }))
    }

    pub fn handle(&self) -> RuntimeHandle {
        RuntimeHandle(Arc::downgrade(&self.0))
    }

    pub fn id(&self) -> RuntimeId {
        RuntimeId(Arc::as_ptr(&self.0) as usize)
    }

    pub fn spawn<T>(&self, future: T) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send,
    {
        self.0.scheduler.spawn(future)
    }

    pub fn block_on<T>(&self, future: T) -> T::Output
    where
        T: Future + Send + 'static,
        T::Output: Send,
    {
        self.0.scheduler.block_on(future)
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
        self.state().scheduler.spawn(future)
    }

    pub fn id(&self) -> RuntimeId {
        RuntimeId(self.0.as_ptr() as usize)
    }

    pub(crate) fn state(&self) -> Arc<RuntimeState> {
        self.0.upgrade().expect("the runtime has been dropped")
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct RuntimeState {
    pub scheduler: Scheduler,
    pub timer_handle: TimerHandle,

    #[cfg(feature = "net")]
    pub network_handle: NetworkHandle,
}

////////////////////////////////////////////////////////////////////////////////

pub struct ContextManager {
    handle: RuntimeHandle,
}

impl ContextManager {
    pub fn install(&self) {
        RUNTIME_HANDLE.with(|h| {
            *h.borrow_mut() = Some(self.handle.clone());
        });
    }

    pub fn enter(&self) -> ContextGuard {
        self.install();
        ContextGuard {}
    }
}

pub struct ContextGuard {}

impl Drop for ContextGuard {
    fn drop(&mut self) {
        RUNTIME_HANDLE.with(|h| {
            *h.borrow_mut() = None;
        });
    }
}
