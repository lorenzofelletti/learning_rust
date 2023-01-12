use log::info;
use std::cmp::min_by;
use std::num::NonZeroUsize;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// A ThreadPool that manages a variable number of threads.
/// The maximum number of threads however cannot exceed the number of available threads on the system.
///
/// # Note
/// You may want to call drop explicitly soon as you are done with the thread pool, otherwise
/// unexpected behaviour may occur.
pub struct ThreadPool {
    /// Vector of worker threads
    workers: Vec<Worker>,
    /// Channel to send jobs to the threads
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // panic if size 0
        assert!(size > 0, "Size must be greater than 0");

        // min between os available threads and size
        let max_threads = min_by(
            thread::available_parallelism().unwrap(),
            NonZeroUsize::new(size).unwrap(),
            |x: &NonZeroUsize, y: &NonZeroUsize| x.cmp(y),
        )
        .get();

        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..max_threads {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Execute a function in the thread pool.
    /// The function will be executed in one of the threads in the pool.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Message::NewJob(Box::new(f));

        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for w in &mut self.workers {
            if let Some(thread) = w.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    _id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    info!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    info!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            _id: id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Terminate,
    NewJob(Job),
}
