fn main() {
    // Each value in Rust has a variable that's called it's owner
    // There can only be one owner at a time.
    // When the owner goes out od scope, the value will be dropped.

    let s = "Hello";

    {
        let s = String::from("Bye");
        // it remains valid until it goes out of scope
    }

    println!("{}", s);

    // Multiple variables can interact with the same data
    let s1 = String::from("Hello");
    let s2 = s1;

    // error from using the invalidated reference
    // println!("{}, world", s1);
    // s1 was moved into s2
    println!("{}, world", s2);

    // Ways Variables and Data Interact: Clone
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack-Only Data: Copy
    // Integers that have a known size at compile time are stored entirely on the stack, so
    // copies of the actual values are quick to make, calling clone wouldn’t do anything
    // different
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Ownership and Functions
    let s1 = String::from("Hello");
    takes_ownership(s1);
    // error value borrowed after move
    // println!("{}", s);

    let s2 = String::from("Hello");
    let (s3, len) = takes_and_gives_back_with_len(s2);
    println!("s3: {} with len: {}", s3, len);
}

fn takes_ownership(s: String) {
    println!("Takes ownership of {}", s);
}

fn takes_and_gives_back_with_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
