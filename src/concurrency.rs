use std::sync::mpsc;
use std::sync::mpsc::{Sender, TryRecvError};
use std::thread;
use std::time::Duration;

pub fn try_threads() {
    let thread_1 = thread::spawn(|| {
        println!("Hello from thread 1!");
        thread::sleep(Duration::from_secs(1));
        println!("Done in thread 1!");
    });
    let thread_2 = thread::spawn(move || {
        println!("Hello from thread 2!");
        thread_1.join().unwrap();
        println!("Doing more work in thread 2!");
        thread::sleep(Duration::from_secs(1));
        println!("Done in thread 2!");
    });
    println!("Hello from local thread!");
    thread_2.join().unwrap();
    println!("Done in local thread!");
}

pub fn try_messaging() {
    let (sender, receiver) = mpsc::channel();
    let sender_clone = sender.clone();

    let sender_1 = thread::spawn(move || {
        println!("Sending from sender 1...");
        thread::sleep(Duration::from_secs(1));
        sender.send(32).unwrap();
        println!("Sent from sender 1...");
    });
    let sender_2 = thread::spawn(move || {
        println!("Sending from sender 2...");
        thread::sleep(Duration::from_secs(2));
        sender_clone.send(24).unwrap();
        println!("Sent from sender 2...");
    });
    let receiver_1 = thread::spawn(move || {
         loop {
            let received = receiver.try_recv();
            match received {
                Ok(result) => {
                    println!("Received {}", result);
                }
                Err(_) => {}
            }
        }
    });
    receiver_1.join().unwrap();

}