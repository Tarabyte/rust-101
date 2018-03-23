use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

enum Message {
    RunJob(Job),
    Shutdown,
}

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl ThreadPool {
    pub fn new(capacity: usize) -> ThreadPool {
        assert!(capacity > 0);

        let (sender, receiver) = channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..capacity)
            .map(|id| {
                Worker::new(id, Arc::clone(&receiver))
            })
            .collect::<Vec<_>>();

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

        self.sender.send(Message::RunJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        (0..self.workers.len()).for_each(|_| {
            self.sender.send(Message::Shutdown).unwrap();
        });

        self.workers.iter_mut().for_each(|worker| {
            println!("Terminating worker {}", worker.id);

           worker.thread.take().map(|thread| {
               thread.join().unwrap();
           });
        })
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                match job {
                    Message::RunJob(job) => {
                        println!("Worker {} got a job. Executing...", id);

                        job.call_box();

                        println!("Worker {} done.", id);
                    },

                    Message::Shutdown => {
                        println!("Worker {} received kill signal.", id);
                        break;
                    },
                }


            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

