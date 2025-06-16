use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use tauri::AppHandle;
use tauri::Emitter;

// Define different event types (must be Send + Sync + 'static)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Event {
    Log { message: String },
    UserAction { user_id: u32, action: String },
    SystemAlert { code: u16, description: String },
}

// A thread-safe, blocking event queue.
// It uses a Condvar to allow the consumer thread to sleep
// until an event is available.
#[derive(Clone)]
pub struct ThreadSafeEventQueue {
    // The Arc allows multiple owners (main thread, consumer thread).
    // The Mutex protects the VecDeque.
    // The Condvar signals when the queue is no longer empty.
    inner: Arc<(Mutex<VecDeque<Event>>, Condvar)>,
}

impl ThreadSafeEventQueue {
    pub fn new() -> Self {
        Self {
            inner: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
        }
    }

    // Adds an event and notifies one waiting thread.
    pub fn enqueue(&self, event: Event) {
        let (lock, cvar) = &*self.inner;
        let mut queue = lock.lock().unwrap();
        queue.push_back(event);
        // Notify the consumer thread that a new event has arrived.
        cvar.notify_one();
    }

    // Waits for an event to be available and returns it.
    // This method will block the calling thread until an event is enqueued.
    pub fn dequeue(&self) -> Event {
        let (lock, cvar) = &*self.inner;
        let mut queue = lock.lock().unwrap();

        // Use a loop to handle spurious wakeups.
        // The `wait` method atomically unlocks the mutex and waits.
        // When woken, it re-locks the mutex.
        while queue.is_empty() {
            queue = cvar.wait(queue).unwrap();
        }

        // At this point, the queue is guaranteed to not be empty.
        queue.pop_front().unwrap()
    }

    pub fn len(&self) -> usize {
        let (lock, _) = &*self.inner;
        lock.lock().unwrap().len()
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

            println!("Consumer processing event: {:?}", event);

            // Sleep for 500ms
            thread::sleep(std::time::Duration::from_millis(500));

            println!(
                "Consumer finished processing event, events left: {:?}",
                queue.len()
            );

            // Example: Emit the processed event to the frontend
            app_handle.emit("rust-event-processed", event).unwrap();
        }
    });
}
