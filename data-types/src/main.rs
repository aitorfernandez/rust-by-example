fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let (x, y, z) = tup;
    let (_, y, _) = tup;

    println!("Then value of y is: {}", y);

    let five_hundred = tup.0;
    println!("The value is: {}", five_hundred);

    // Array type
    // let a = [1, 2, 3, 4];
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // same as let a = [3, 3, 3, 3, 3];

    println!("The 1st value is: {}", a[0]);
}
