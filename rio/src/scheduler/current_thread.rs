use crate::runtime::ContextManager;

use super::task::{Task, TaskId};

use futures::task::ArcWake;
use log::debug;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex, MutexGuard, Weak},
    task::{Context, Poll},
    thread::{self, Thread},
};

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct RunQueue {
    queue: VecDeque<TaskId>,
    set: HashSet<TaskId>,
}

impl RunQueue {
    pub fn pop_front(&mut self) -> Option<TaskId> {
        self.queue.pop_front().map(|task_id| {
            assert!(self.set.remove(&task_id));
            task_id
        })
    }

    pub fn push_back(&mut self, task_id: TaskId) -> bool {
        let ok = self.set.insert(task_id);
        if ok {
            self.queue.push_back(task_id);
        }
        ok
    }
}

////////////////////////////////////////////////////////////////////////////////

struct SharedState {
    run_queue: RunQueue,
    thread: Thread,
}

////////////////////////////////////////////////////////////////////////////////

pub struct CurrentThreadScheduler {
    context_manager: ContextManager,
    tasks: Mutex<HashMap<TaskId, Task>>,
    shared_state: Arc<Mutex<SharedState>>,
}

impl CurrentThreadScheduler {
    pub fn new(context_manager: ContextManager) -> Self {
        Self {
            context_manager,
            tasks: Default::default(),
            shared_state: Arc::new(Mutex::new(SharedState {
                run_queue: RunQueue::default(),
                thread: thread::current(),
            })),
        }
    }

    pub fn submit(&self, task: Task) {
        self.emplace_task(task);
    }

    pub fn run_until_done(&self, task: Task) {
        let root_task_id = task.id();
        self.emplace_task(task);

        self.lock_shared_state().thread = thread::current();
        let _guard = self.context_manager.enter();

        loop {
            let mb_task = 'find_task: {
                let mut state = self.lock_shared_state();
                let mut tasks = self.lock_tasks();

                while let Some(task_id) = state.run_queue.pop_front() {
                    if let Some(task) = tasks.remove(&task_id) {
                        break 'find_task Some(task);
                    }
                }
                None
            };

            let Some(mut task) = mb_task else {
                thread::park();
                continue;
            };

            debug!("polling task #{}", task.id());
            if self.poll_task(&mut task).is_pending() {
                self.lock_tasks().insert(task.id(), task);
            } else if task.id() == root_task_id {
                return;
            }
        }
    }

    fn emplace_task(&self, task: Task) {
        let task_id = task.id();
        let prev_task = self.lock_tasks().insert(task_id, task);
        assert!(prev_task.is_none(), "duplicate task id: {}", task_id);

        debug!("emplacing task #{}", task_id);

        let mut state = self.lock_shared_state();
        state.run_queue.push_back(task_id);
        state.thread.unpark();
    }

    fn poll_task(&self, task: &mut Task) -> Poll<()> {
        let waker = futures::task::waker(Arc::new(Waker {
            shared_state: Arc::downgrade(&self.shared_state),
            task_id: task.id(),
        }));
        let mut context = Context::from_waker(&waker);
        task.poll(&mut context)
    }

    fn lock_shared_state(&self) -> MutexGuard<'_, SharedState> {
        self.shared_state
            .lock()
            .expect("failed to lock shared state in scheduler")
    }

    fn lock_tasks(&self) -> MutexGuard<'_, HashMap<TaskId, Task>> {
        self.tasks.lock().expect("failed to lock tasks")
    }
}

////////////////////////////////////////////////////////////////////////////////

struct Waker {
    shared_state: Weak<Mutex<SharedState>>,
    task_id: TaskId,
}

impl ArcWake for Waker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let Some(state_mutex) = arc_self.shared_state.upgrade() else {
            debug!("failed to wake task #{}: no scheduler", arc_self.task_id);
            return;
        };

        debug!("waking task #{}", arc_self.task_id);
        let mut state = state_mutex.lock().expect("failed to lock state in waker");
        state.run_queue.push_back(arc_self.task_id);
        state.thread.unpark();
    }
}
