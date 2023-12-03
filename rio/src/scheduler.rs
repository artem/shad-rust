use std::{
    collections::{HashMap, VecDeque},
    pin::Pin,
    sync::{Arc, Mutex, MutexGuard},
    task::Context,
    thread::{self, Thread},
};

use futures::{task::ArcWake, Future};
use log::debug;

////////////////////////////////////////////////////////////////////////////////

pub type TaskId = u64;
type BoxedTask = Pin<Box<dyn Future<Output = ()> + Send>>;

struct SchedulerState {
    tasks: HashMap<TaskId, BoxedTask>,
    run_queue: VecDeque<TaskId>,
    next_id: u64,
    thread: Thread,
}

impl Default for SchedulerState {
    fn default() -> Self {
        Self {
            tasks: Default::default(),
            run_queue: Default::default(),
            next_id: 0,
            thread: thread::current(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Scheduler {
    state: Arc<Mutex<SchedulerState>>,
}

impl Scheduler {
    pub fn submit<T>(&self, task: T) -> TaskId
    where
        T: Future<Output = ()> + Send + 'static,
    {
        let boxed_task = Box::pin(task);

        let task_id = {
            let mut state = self.lock_state();

            let task_id = state.next_id;
            state.next_id += 1;

            let prev_task = state.tasks.insert(task_id, boxed_task);
            assert!(prev_task.is_none(), "duplicate task id in scheduler");

            state.run_queue.push_back(task_id);
            state.thread.unpark();
            task_id
        };

        debug!("submitted task #{}", task_id);
        task_id
    }

    pub fn block_on(&self, root_task_id: TaskId) {
        {
            let mut state = self.lock_state();
            assert!(state.tasks.contains_key(&root_task_id));
            state.thread = thread::current();
        }
        loop {
            let mb_task = 'find_task: {
                let mut state = self.lock_state();
                while let Some(task_id) = state.run_queue.pop_front() {
                    if let Some(task) = state.tasks.remove(&task_id) {
                        break 'find_task Some((task_id, task));
                    }
                }
                None
            };

            let Some((task_id, mut task)) = mb_task else {
                thread::park();
                continue;
            };

            let waker = futures::task::waker(Arc::new(Waker {
                scheduler_state: self.state.clone(),
                task_id,
            }));
            let mut context = Context::from_waker(&waker);

            debug!("polling task #{}", task_id);
            if task.as_mut().poll(&mut context).is_pending() {
                self.lock_state().tasks.insert(task_id, task);
            } else if task_id == root_task_id {
                return;
            }
        }
    }

    fn lock_state(&self) -> MutexGuard<'_, SchedulerState> {
        self.state
            .lock()
            .expect("failed to lock scheduler state in scheduler")
    }
}

////////////////////////////////////////////////////////////////////////////////

struct Waker {
    scheduler_state: Arc<Mutex<SchedulerState>>,
    task_id: TaskId,
}

impl ArcWake for Waker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        debug!("waking task #{}", arc_self.task_id);
        let mut state = arc_self
            .scheduler_state
            .lock()
            .expect("failed to lock scheduler state in waker");
        state.run_queue.push_back(arc_self.task_id);
        state.thread.unpark();
    }
}
