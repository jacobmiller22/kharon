use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
#[derive(Debug)]
pub struct PoolCreationError {
    details: String
}

impl PoolCreationError {
  fn new(msg: &str) -> PoolCreationError {
    PoolCreationError{details: msg.to_string()}
  }
}
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
  /// Create a new ThreadPool
  /// size: number of threads
  /// PoolCreationError when size <= 0
  pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    if size <= 0 {
      return Err(PoolCreationError::new("Invalid Size"));
    }

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers:Vec<Worker> = Vec::with_capacity(size);
    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    return Ok(ThreadPool {workers, sender});
  }

  pub fn execute<F>(&self, f: F)
  where
      F: FnOnce() + Send + 'static,
  {
      let job = Box::new(f);

      self.sender.send(job).unwrap();
  }
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move|| loop { 
      let job = receiver.lock().unwrap().recv().unwrap();

      job();
    });

    Worker {id, thread}
  }
}