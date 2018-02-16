use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver};

fn main() {
    test_threads_join(10);

    test_message_passing();

    test_passing_multiple_times();

    test_multiple_senders();
}

fn test_multiple_senders() {
    fn report(thread_number: u64, tx: &Sender<String>) {
        let tx = Sender::clone(tx);

        thread::spawn(move || {
            for i in 1..10 {
                tx.send(format!("Message {} from thread {}.", i, thread_number)).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    fn spawn(num: u8) -> Receiver<String> {
        let (tx, rx) = channel();

        for i in 1..num + 1 {
            report(i as u64, &tx)
        }

        rx
    }


    let rx = spawn(3);


    for message in rx {
        println!("Got message: `{}`", message);
    }
}

fn test_passing_multiple_times() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let v = vec![1, 2, 3];

        for i in v {
            thread::sleep(Duration::from_millis(100));
            println!("Sending {}", i);
            tx.send(i).unwrap();

        }
    });

    for item in rx {
        println!("received {}", item);
    }
}

fn test_message_passing() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let v = vec![1, 2, 3];

        tx.send(v).unwrap();
    });

    let v = rx.recv().unwrap();

    println!("{:?}", v);
}

fn test_threads_join(delay: u64) {

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("thread {}", i);
            thread::sleep(Duration::from_millis(delay));
        }
    });

    for i in 1..10 {
        println!("main {}", i);
        thread::sleep(Duration::from_millis(delay));
    }

    handle.join().unwrap();
}
