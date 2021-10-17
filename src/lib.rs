use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::net::{TcpListener, TcpStream, SocketAddr};

///
/// Thread pooling implemented in accordance to:
/// https://doc.rust-lang.org/book/ch20-02-multithreaded.html
/// Set a limit on the amount of threads that are allowed to 
/// be spawned.
/// 
#[allow(dead_code)]
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)))
    }

    ThreadPool {
      workers,
      sender,
    }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);
    match self.sender.send(job) {
      Ok(_) => {
        println!("Assigning job...");
      }
      Err(e) => {
        panic!("An error has occured: {}", e);
      }
    }
  }
}

#[allow(dead_code)]
struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap();

      println!("Worker {} got a job; executing.", id);

      job();
    });

    Worker { id, thread }
  }
}

/// A Node is a listening device

#[allow(dead_code)]
pub struct Node {
  listener: TcpListener,
  pool: ThreadPool
}

impl Node {
  pub fn new(ip: &str, port: u32, pool_size: usize) -> Node {
    let pool = ThreadPool::new(pool_size);

    let listener = match TcpListener::bind(format!("{}:{}", ip, port)) {
        Ok(listener) => { 
            println!("Listening on port {}", port);
            listener
        }
        Err(e) => panic!("Unable to open connection: {}", e)
    };

    Node {
      listener,
      pool
    }
  }

  pub fn serve(&self) -> (TcpStream, SocketAddr) {
    match self.listener.accept() {
      Ok(listener) => listener,
      Err(e) => panic!("Unable to establish server: {}", e)
    }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static
  {
    self.pool.execute(f)
  }
}