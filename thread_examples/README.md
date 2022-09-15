## Threads
In Rust, threads are represented by the `thread` module.
You can import the thread module with `use std::thread;`.
The `thread` module provides a `spawn` function, that creates a new thread.
The `spawn` function takes a closure as an argument.

When the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have completed their work.

### Waiting for a thread to finish
The `thread` module also provides a `join` method, that takes a `JoinHandle` as an argument.
A `JoinHandle` is a value returned from the `spawn` function.
The `join` method waits for the thread to finish.
Example:
```Rust
// Create a new thread
let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
});

// do something in main thread

// Wait for the thread to finish.
handle.join().unwrap();
```
