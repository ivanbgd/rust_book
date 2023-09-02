use std::any::{Any, TypeId};
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    hi_and_vec();
    vector();
    // channels();
    client_server();
    shared_state();
    // deadlock();
    // arc_reference();
}

fn hi_and_vec() {
    println!();

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi, number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        println!("Here's Johnny: {:?}!", v);
    });

    // handle2.join().unwrap();

    for i in 1..5 {
        println!("Hi, number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    handle2.join().unwrap();
}

fn vector() {
    println!();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's Johnny: {:?}!", v);
    });

    handle.join().unwrap();
}

fn channels() {
    println!();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let value = String::from("Hi!");
        tx.send(value).unwrap();
    });
    let received = rx.recv().unwrap();  // rx.recv() always blocks.
    println!("Main thread got: {}", received);

    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let value = String::from("Hi, again!");
        tx.send(value).unwrap();
    });
    handle.join().unwrap();
    let received = rx.try_recv().unwrap();  // rx.try_recv() never blocks.
    println!("Main thread got: {}", received);

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let value = String::from("And again, hi!");
        tx.send(value.clone()).unwrap();  // We need to clone it.
        println!("\tValue is: '{}'", value);
    });
    let received = rx.recv_timeout(Duration::from_millis(1)).unwrap();  // rx.recv_timeout() always blocks.
    println!("Main thread got: {}", received);

    println!();
    let (tx, rx) = mpsc::channel();

    // Send multiple messages from a single producer.
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    // We are treating `rx` as an iterator. When the channel is closed, iteration will end.
    for received in rx {
        println!("Got: {}", received);
    }

    println!();
    let (tx, rx) = mpsc::channel();

    // Multiple producers (multiple threads)
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn client_server() {
    println!();
    enum ClientMessage { Incr, Get, Quit }
    enum ServerMessage { Get(usize) }

    let (server_tx, client_rx) = mpsc::channel();
    let (client_tx, server_rx) = mpsc::channel();

    let server = thread::spawn(move || {
        let mut n = 0;
        loop {
            match server_rx.recv().unwrap() {
                ClientMessage::Incr => n += 1,
                ClientMessage::Get => server_tx.send(ServerMessage::Get(n)).unwrap(),
                ClientMessage::Quit => break,
            }
        }
    });

    for msg in [ClientMessage::Incr, ClientMessage::Get, ClientMessage::Quit] {
        client_tx.send(msg).unwrap();
    }

    let ServerMessage::Get(n) = client_rx.recv().unwrap();
    // 1; TypeId { t: 13834754221672687376 }, TypeId { t: 13834754221672687376 }
    println!("{}; {:?}, {:?}", n, n.type_id(), TypeId::of::<usize>());

    server.join().unwrap();
}

fn shared_state() {
    println!();
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();  // blocks
        // let mut num2 = m.lock().unwrap();  // blocks - deadlocks!
        println!("num = {:?}", num);  // 5
        *num += 1;
        println!("num = {:?}", num);  // 6
        *num += 1;
        println!("num = {:?}", num);  // 7
    }
    println!("m = {:?}", m);  // m = Mutex { data: 7, poisoned: false, .. }
    println!("m = {:?}", m.lock());  // m = Ok(7)
    println!("m = {}", *m.lock().unwrap());  // m = 7

    // Using an Arc<T> to wrap the Mutex<T> to be able to share ownership across multiple threads
    println!();
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            *counter.lock().unwrap() += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter = {}", *counter.lock().unwrap());
}

/// It deadlocks with using either `lock()`, which blocks, or `try_lock()`, which doesn't block.
/// Flags `A` and `B` exist to enforce something. This is, of course, a toy example.
/// I was able to make an example code that deadlocks.
/// TODO: The next step would be to solve the deadlocking.
fn deadlock() {
    println!();

    let flag_a = Arc::new(Mutex::new(false));
    let flag_b = Arc::new(Mutex::new(false));
    let x = Arc::new(Mutex::new(0));

    let flag_a_clone1 = Arc::clone(&flag_a);
    let flag_a_clone2 = Arc::clone(&flag_a);
    let flag_b_clone1 = Arc::clone(&flag_b);
    let flag_b_clone2 = Arc::clone(&flag_b);
    let x_clone1 = Arc::clone(&x);
    let x_clone2 = Arc::clone(&x);

    let handle1 = thread::spawn(move || {
        println!("1-1: flag_a_clone1: {:?}, flag_b_clone1: {:?}, x_clone1: {:?}", flag_a_clone1, flag_b_clone1, x_clone1);
        loop {
            if *flag_a_clone1.try_lock().unwrap() {
                *x_clone1.lock().unwrap() += 1;
                *flag_b_clone1.lock().unwrap() = true;
                println!("1-2: flag_a_clone1: {:?}, flag_b_clone1: {:?}, x_clone1: {:?}", flag_a_clone1, flag_b_clone1, x_clone1);
                break;
            }
        }
        println!("1-3: flag_a_clone1: {:?}, flag_b_clone1: {:?}, x_clone1: {:?}", flag_a_clone1, flag_b_clone1, x_clone1);
    });

    let handle2 = thread::spawn(move || {
        println!("2-1: flag_a_clone2: {:?}, flag_b_clone2: {:?}, x_clone2: {:?}", flag_a_clone2, flag_b_clone2, x_clone2);
        loop {
            if *flag_b_clone2.try_lock().unwrap() {
                *x_clone2.lock().unwrap() -= 10;
                *flag_a_clone2.lock().unwrap() = true;
                println!("2-2: flag_a_clone2: {:?}, flag_b_clone2: {:?}, x_clone2: {:?}", flag_a_clone2, flag_b_clone2, x_clone2);
                break;
            }
        }
        println!("2-3: flag_a_clone2: {:?}, flag_b_clone2: {:?}, x_clone2: {:?}", flag_a_clone2, flag_b_clone2, x_clone2);
    });

    // This is necessary, as main thread is also a thread, so we have three threads in total.
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Expected to be: -9.
    println!("x = {}", *x.lock().unwrap());
    assert_eq!(-9, *x.lock().unwrap());

    // Expected to be: "true, true".
    println!("flag_a: {:?}, flag_b {:?}", flag_a, flag_b);
}

// /// Doesn't compile: `s` does not live long enough.
// fn arc_reference() {
//     let s = String::from("Hello world");
//     let a = Arc::new(&s);
//     let a2 = Arc::clone(&a);
//     let t = thread::spawn(move || a2.len());
//     let len = t.join().unwrap();
//     println!("{} {}", a, len);
// }
