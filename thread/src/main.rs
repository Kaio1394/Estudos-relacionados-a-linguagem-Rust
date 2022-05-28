use crate::thread::sleep;
use std::{thread, time::Duration};

fn main() {
    let mut threads = vec![];
    for i in 0..10{
        let th = thread::spawn(move || {
            sleep(Duration::from_millis(i * 1000));
            println!("New thread {}", i);
        });
        threads.push(th);
    }

    #[allow(unused_must_use)]
    for t in threads{
        t.join();
    }
    println!("Main thread.");
}
