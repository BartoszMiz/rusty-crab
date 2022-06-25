
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
	pub fn new(count: usize) -> Self {
		assert!(count > 0);

		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));

		let mut workers = Vec::with_capacity(count);
		for id in 0..count {
			workers.push(Worker::new(id, receiver.clone()));
		}
		Self { workers, sender }
	}

	pub fn execute<F>(&self, f: F)
	where F: FnOnce() + Send + 'static
	{
		let job = Box::new(f);
		self.sender.send(job).unwrap();
	}
}

struct Worker {
	id: usize,
	thread: thread::JoinHandle<()>
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
		let thread = thread::spawn(move || loop {
			let job = receiver.lock().unwrap().recv().unwrap();
			job();
		});
		Worker { id, thread }
	}
}
