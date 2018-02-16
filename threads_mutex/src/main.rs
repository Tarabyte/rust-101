use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    test_mutex_debug();

    test_mutex_mutability();

    test_mutex_in_threads();
}

fn test_mutex_in_threads() {
    let m = Arc::new(Mutex::new(0));

    (0..10).for_each(|i| {
        let m = Arc::clone(&m);

        thread::spawn(move || {
            let mut counter = m.lock().unwrap();

            *counter += i;
        }).join().unwrap();
    });

    println!("{:?}", m);
}

fn test_mutex_debug() {
    let m = Mutex::new(5);

    println!("{:?}", m);
}

fn test_mutex_mutability() {
    let m = Mutex::new(10);

    {
        let mut m = m.lock().unwrap();

        *m += 10;
    }

    println!("{:?}", m);
}