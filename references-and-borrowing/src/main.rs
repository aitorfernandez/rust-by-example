fn main() {
    let s = String::from("Hello");
    let len = calculate_length(&s);

    println!("The length of '{}' is {}.", s, len);

    // change(&s);

    let mut s = String::from("Hello");

    change(&mut s);

    println!("{}", s);

    // One mutable reference to a particular piece of data
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
