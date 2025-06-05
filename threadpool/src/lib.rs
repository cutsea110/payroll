use log::{debug, info, trace};
use std::{
    sync::mpsc,
    sync::{Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        trace!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender
                .send(Message::Terminate)
                .expect("Send terminate message to worker");
        }

        debug!("Shutting down all workers.");

        for worker in &mut self.workers {
            debug!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().expect("join thread");
            }
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        trace!("ThreadPool: execute");
        let job = Box::new(f);

        debug!("Sending new job message to worker");
        self.sender
            .send(Message::NewJob(job))
            .expect("Send new job message to worker");
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            trace!("thread spawned");
            let message = receiver
                .lock()
                .expect("lock receiver")
                .recv()
                .expect("receive message");
            trace!("thread got message");
            match message {
                Message::NewJob(job) => {
                    info!("Worker {} got a job; executing.", id);

                    job.call_box();
                }
                Message::Terminate => {
                    info!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
