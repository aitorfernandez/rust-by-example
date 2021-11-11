fn main() {
    let s = String::from("Hello");

    let slice = &s[0..2]; // or &s[..2]
    println!("{}", slice); // He

    let slice = &s[3..s.len()]; // or &s[3..]
    println!("{}", slice); // lo

    let slice = &s[0..s.len()]; // or &s[..]
    println!("{}", slice); // Hello

    let s = String::from("Hello world");
    let word = first_word(&s);

    println!("The first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // println!("{}", item); // 72, 101, 108 ...
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
