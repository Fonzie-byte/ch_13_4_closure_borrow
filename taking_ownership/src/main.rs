fn main() {
    let nums = vec![1, 2, 3];
    println!("Before closure declaration:\t{nums:?}");

    let takes_ownership = move || println!("From within the closure:\t{nums:?}");

    takes_ownership();
}
