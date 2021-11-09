fn main() {
    // cannot assign twice to immutable variable `x`
    // let x = 5;
    // print!("The value of x is: {}", x);
    // x = 6;
    // print!("The value of x is: {}", x);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants can be declared in any scope
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    // Declare a variable with the same name
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // ok creating a new variable with different value
    // let spaces = "    ";
    // let spaces = spaces.len();

    // error different types
    // let spaces = "    ";
    // spaces = spaces.len();
}
