use std::thread;

fn main() {
    let nums = vec![1, 2, 3];
    println!("Before closure declaration:\t{nums:?}");

    /*
    // Most basic example that moves a value.
    let takes_ownership = move || println!("From within the closure:\t{nums:?}");

    takes_ownership();
    */

    // Moving is especially common when moving to another thread.
    thread::spawn(move || println!("From within closure:\t{nums:?}"))
        .join() // Waits for the thread to finish and join back with the main process.
        .unwrap();
}
