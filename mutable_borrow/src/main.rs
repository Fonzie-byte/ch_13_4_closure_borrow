fn main() {
    let mut nums = vec![1, 2, 3];
    println!("Before closure declaration:\t{nums:?}");

    let mut mutable_borrow = || nums.push(7);

    mutable_borrow();
    println!("After calling the closure:\t{nums:?}");
}
