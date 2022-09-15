use std::thread;
use std::time::Duration;

fn main() {
    println!("---- FIRST EXAMPLE START ----");
    simple_thread_example();
    println!("---- FIRST EXAMPLE END ----");

    println!("---- SECOND EXAMPLE START ----");
    wait_for_thread_to_finish();
    println!("---- SECOND EXAMPLE END ----");
}

/// This example shows how to create a thread.
fn simple_thread_example() {
    // Create a new thread
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

/// This example shows how to wait for a thread to finish.
fn wait_for_thread_to_finish() {
    /*
    A JoinHandle is an owned value that, when we call the join method on it,
    will wait for its thread to finish.
    */
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for the thread to finish.
    handle.join().unwrap();
}
