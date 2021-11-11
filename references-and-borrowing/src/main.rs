fn main() {
    let s = String::from("Hello");
    let len = calculate_length(&s);

    println!("The length of '{}' is {}.", s, len);

    // change(&s);

    let mut s = String::from("Hello");

    change(&mut s);

    println!("{}", s);

    // One mutable reference to a particular piece of data
    let mut s = String::from("Hello");

    // error two mutable borrow
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // ok
    {
        let r1 = &mut s;
    }

    let r2 = &mut s;

    // Don't combine mutable and inmutable references.
    // error
    // let mut s = String::from("Hello");

    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;

    // println!("{}, {}, {}", r1, r2, r3);

    // ok
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;

    println!("{}", r3);

    // Dangling references
    let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
//     // s goes outh of scope
// }

fn dangle() -> String {
    let s = String::from("Hello");
    s
    // Ownership is moved out
}

// s is a reference but no ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}

// error, variable are inmutable by default, so are references.
// fn change(s: &String) {
//     s.push_str(", world");
// }

// ok
fn change(s: &mut String) {
    s.push_str(", world");
}
