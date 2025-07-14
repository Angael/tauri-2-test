use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Arc, Condvar};
use std::thread;
use tauri::AppHandle;

use crate::state_manager::JsonState;
use crate::task_queue::task::Task;
use crate::task_queue::task_handlers::handle_task_generate_thumb;

// A thread-safe, blocking event queue.
#[derive(Clone)]
pub struct ThreadSafeEventQueue {
    inner: JsonState<VecDeque<Task>>,

    // The Condvar signals when the queue is no longer empty.
    cvar: Arc<Condvar>,
}

// impl Default for ThreadSafeEventQueue {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl ThreadSafeEventQueue {
    pub fn new(path: PathBuf) -> Self {
        Self {
            inner: JsonState::load(path),
            cvar: Arc::new(Condvar::new()),
        }
    }

    // Adds an event and notifies one waiting thread.
    pub fn enqueue(&self, event: Task) {
        let cvar = &*self.cvar;
        // let mut queue = lock.lock().unwrap();
        let _ = self.inner.with_mut(|queue| {
            queue.push_back(event);
        });
        // Notify the consumer thread that a new event has arrived.
        cvar.notify_one();
    }

    // Waits for an event to be available and returns it.
    // This method will block the calling thread until an event is enqueued.
    pub fn dequeue(&self) -> Task {
        let cvar = &*self.cvar;
        let mut queue = self.inner.raw_state().lock().unwrap();

        // Use a loop to handle spurious wakeups.
        // The `wait` method atomically unlocks the mutex and waits.
        // When woken, it re-locks the mutex.
        while queue.is_empty() {
            queue = cvar.wait(queue).unwrap();
        }

        drop(queue); // Drop the lock before locking with with_mut

        self.inner
            .with_mut(|q| {
                // This will block until an event is available.
                q.pop_front().unwrap()
            })
            .unwrap()
    }

    pub fn len(&self) -> usize {
        return self.inner.with(|queue| queue.len());
    }

    pub fn is_empty(&self) -> bool {
        return self.inner.with(|queue| queue.is_empty());
    }

    pub fn force_save_blocking(&self) -> Result<(), String> {
        self.inner.force_save_blocking()
    }
}

// The consumer thread function.
pub fn start_event_consumer(queue: ThreadSafeEventQueue, app_handle: AppHandle) {
    thread::spawn(move || {
        println!("Event consumer thread started.");

        // This loop will run forever.
        loop {
            // The call to `dequeue` will block here until an event is
            // available, consuming no CPU while waiting.
            let event = queue.dequeue();

            match event {
                Task::GenerateThumb(task) => {
                    handle_task_generate_thumb(task, &app_handle);
                }
            }
        }
    });
}
