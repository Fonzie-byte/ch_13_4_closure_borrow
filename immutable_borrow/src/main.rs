fn main() {
    let nums = vec![1, 2, 3];
    println!("Before closure declaration:\t{nums:?}");

    let immutable_borrow = || println!("From within the closure:\t{nums:?}");

    println!("Before calling the closure::\t{nums:?}");
    immutable_borrow();
    println!("After calling the closure:\t{nums:?}");
}
