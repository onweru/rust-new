// use std::thread;
// use std::time::Duration;
// use std::sync::mpsc;
// use std::sync::Mutex;
// use std::rc::Rc
use std::{
    sync::{Arc, mpsc, Mutex,},
    thread,
    time::Duration,
};

fn main() {
    spawn_threads();

    try_channels();

    send_text_via_channels();

    try_mutex();

    increment_using_mutex();

}

fn spawn_threads() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread! {:?}", i, v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi humber {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn try_channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn send_text_via_channels() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("check"),
        ];

        for val in vals {
            tx1.send(val).expect("expected message");
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hola"),
            String::from("donde"),
            String::from("el"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Tengo: {}", received);
    }
}

fn try_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

}

fn increment_using_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}