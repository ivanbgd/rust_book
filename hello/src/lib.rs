//! The `ThreadPool` library
//!
//! This library can be used to create a pool of threads.
//!
//! The pool can be used in a web server, as with our example here,
//! but also for other purposes.

mod error_consts;

use std::any::type_name;
use std::fmt::{Debug, Display, Formatter};
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

use error_consts::*;

/// Create a `ThreadPool`
///
/// Contains several different implementations for practice and as examples.
/// Some implementations may panic, but the last implementation doesn't panic.
/// It recovers from the error by using a default value for the number of threads.
///
/// This function isn't necessary, as `ThreadPool::new()` and `ThreadPool::build()`
/// are marked public, but serves the educational purpose.
///
/// # Panics
///
/// Potentially panics if `size` is zero; some implementations may panic.
pub fn create_pool(size: usize) -> ThreadPool {
    const NUM_CPU: usize = 4;

    // let pool = ThreadPool::new(size);

    // let pool = match ThreadPool::build(size) {
    //     Ok(p) => p,
    //     Err(e) => panic!("{}", e),
    // };

    // Even though using `expect()` is preferred over using `unwrap()`,
    // we can safely use `unwrap()` here because `ThreadPool::build()`
    // returns `PoolCreationError`, which has the `Debug` and `Display`
    // traits implemented, so it provides the expected output when unwrapped.
    // let pool = ThreadPool::build(size).expect("");

    // let pool = ThreadPool::build(size).unwrap();

    let pool = match ThreadPool::build(size) {
        Ok(p) => p,
        Err(_) => ThreadPool::new(NUM_CPU),
    };

    pool
}

/// A thread pool that executes connections asynchronously
///
/// Only one of `new()` or `build()` functions is necessary,
/// but we are practicing, so we have both.
///
/// Contains a vector of workers and a sender which sends tasks
/// to workers down the channel.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

/// The type of job that threads in the pool execute
type Job = Box<dyn FnOnce() -> () + Send + 'static>;

impl ThreadPool {

    /// Create a new `ThreadPool`
    ///
    /// `size` is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// Panics if `size` is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "{}", ERROR_POOL_CREATION);

        Self::create_threads(size)
    }

    /// Create a new `ThreadPool`
    ///
    /// `size` is the number of threads in the pool.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }

        Ok(Self::create_threads(size))
    }

    /// Take a job and execute it
    ///
    /// Sends the job to a worker down the channel.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() -> () + Send + 'static,
    {
        let job = Box::new(f);

        self.sender
            .as_ref().expect("Expected to extract sender from Some.")
            .send(job).expect("Expected to send a job.");
    }

    /// Inner function with functionality that is common to `new` and `build`
    fn create_threads(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Create threads and store them in the vector
            // Share the receiver among the workers using Arc and Mutex
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
}

impl Drop for ThreadPool {
    /// Used for graceful shutdown of worker threads
    ///
    /// We don't call it explicitly.
    /// It's called implicitly when `ThreadPool` goes out of scope.
    fn drop(&mut self) {
        // Drop sender explicitly before joining the worker threads
        // This closes the channel at the sender's end and consequently overall.
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!(" Shutting down worker {}.", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().expect(format!("Expected to join the worker's {} thread.", worker.id).as_ref());
            }
        }
    }
}

/// A worker thread
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Create a new worker thread
    ///
    /// Takes the worker's ID and a channel receiver through which it
    /// receives jobs that it needs to execute.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let builder = thread::Builder::new();

        // A thread loops forever waiting for jobs, but we have implemented a graceful shutdown.
        // If recv() returns an error, we break out of the loop in a graceful manner.
        // This will happen when the sender is dropped, as that will close the channel.
        // Only after that can threads be joined in a regular way. They couldn't be joined
        // if they were looping infinitely, but we are breaking out of the loop when the
        // channel is closed, so threads can be shut down gracefully.
        let handler_result = builder.spawn(move || {
            loop {
                let message =
                    receiver.lock()
                        .expect(format!("Expected receiver for worker {} to acquire the lock.", id).as_ref())
                        .recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    },
                    Err(_) => {
                        println!("  Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        // This is the same behavior that `thread::spawn()` has, so we haven't accomplished
        // anything by using a `thread::Builder` instead here, in this case.
        // Still, we can use this as a placeholder for future improvements.
        // Concretely, the `Err` arm's output can be improved, to handle an OS error in a better way.
        // We can, for example, try to recover, instead of crashing the program.
        // The program should (potentially) be able to continue working even if OS can't spawn a thread.
        let thread = match handler_result {
            Ok(thread) => thread,
            Err(error) => panic!("Worker error; the OS couldn't spawn a new worker: {}", error),
        };

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[derive(Clone)]
pub struct PoolCreationError;

impl Debug for PoolCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", type_name::<PoolCreationError>(), ERROR_POOL_CREATION)
    }
}

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", type_name::<PoolCreationError>(), ERROR_POOL_CREATION)
    }
}

#[cfg(test)]
mod tests {
    use super::ThreadPool;

    const NUM_CPU_TEST: usize = 4;

    #[test]
    fn test_create_threads() {
        let pool = ThreadPool::create_threads(NUM_CPU_TEST);
        assert_eq!(NUM_CPU_TEST, pool.workers.len());
    }

    #[test]
    #[should_panic]
    fn test_new_zero_threads() {
        let _pool = ThreadPool::new(0);
    }

    #[test]
    fn test_new_four_threads() {
        let pool = ThreadPool::new(NUM_CPU_TEST);
        assert_eq!(NUM_CPU_TEST, pool.workers.len());
    }

    #[test]
    fn test_build_zero_threads() {
        let pool_result = ThreadPool::build(0);
        assert!(pool_result.is_err());
    }

    #[test]
    fn test_build_four_threads() {
        let pool_result = ThreadPool::build(NUM_CPU_TEST);
        assert!(pool_result.is_ok());
        let pool = pool_result.unwrap();
        assert_eq!(NUM_CPU_TEST, pool.workers.len());
    }
}
