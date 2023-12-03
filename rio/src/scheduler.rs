mod current_thread;
mod task;

#[cfg(feature = "rt-multi-thread")]
mod multi_thread;

////////////////////////////////////////////////////////////////////////////////

use current_thread::CurrentThreadScheduler;
use task::Task;

#[cfg(feature = "rt-multi-thread")]
use multi_thread::MultiThreadScheduler;

use futures::{channel::oneshot, FutureExt};
use thiserror::Error;

use std::{
    future::Future,
    pin::Pin,
    sync,
    task::{Context, Poll},
};

use crate::runtime::ContextManager;

////////////////////////////////////////////////////////////////////////////////

pub enum Scheduler {
    CurrentThread(CurrentThreadScheduler),

    #[cfg(feature = "rt-multi-thread")]
    MultiThread(MultiThreadScheduler),
}

impl Scheduler {
    pub fn new_current_thread(context_manager: ContextManager) -> Self {
        Scheduler::CurrentThread(CurrentThreadScheduler::new(context_manager))
    }

    #[cfg(feature = "rt-multi-thread")]
    pub fn new_multi_thread(context_manager: ContextManager, num_threads: usize) -> Self {
        Scheduler::MultiThread(MultiThreadScheduler::new(context_manager, num_threads))
    }

    pub fn spawn<T>(&self, future: T) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send,
    {
        let (sender, receiver) = oneshot::channel();
        let task = Task::from(async move {
            let _ = sender.send(future.await);
        });

        match self {
            Scheduler::CurrentThread(current_thread) => current_thread.submit(task),

            #[cfg(feature = "rt-multi-thread")]
            Scheduler::MultiThread(multi_thread) => multi_thread.submit(task),
        }

        JoinHandle { receiver }
    }

    pub fn block_on<T>(&self, future: T) -> T::Output
    where
        T: Future + Send + 'static,
        T::Output: Send,
    {
        let (sender, receiver) = sync::mpsc::sync_channel(1);
        let task = Task::from(async move {
            let _ = sender.send(future.await);
        });

        match self {
            Scheduler::CurrentThread(current_thread) => current_thread.run_until_done(task),

            #[cfg(feature = "rt-multi-thread")]
            Scheduler::MultiThread(multi_thread) => multi_thread.submit(task),
        }

        receiver.recv().expect("recv() failed in block_on")
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
