#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

fn main() {
    // Type inference
    {
        let foo: u8 = 12; // or let small_number = 12u8; // or let small_number = 12_u8;
        let bar = 96_000_000_i32;

        let baz: f32 = 6.; // or let float_number = 6.0; // or let float_number = 6.0f64; // or float_number = 6.0_f64
        let qux = 12.0; // Rust will choose f64
    }
    // Mutability
    {
        let mut foo = 12;
        foo = 6;
    }
    // Shadowing
    {
        let foo = 12;
        let foo = 24;
    }
    // String and &str
    {
        let foo = "Hello, world!"; // it's a &str

        // String is a pointer with data on the heap
        let bar = "Hello, world!".to_string(); // or String::from("Hello, world!"); // or let bar = format!("{}", foo);

        // into() needs a type
        let baz: String = "Hello, world!".into();
    }
    // Const and statid
    {
        // const for values that don't change
        const NUMBER_OF_MONTHS: u32 = 12;
        // static with fixed memory location
        static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
    }
    // Mutable references
    {
        let mut foo: i32 = 3;
        let bar = &mut foo;
        *bar += 3; // bar is not the i32 value, it's a &i32, the value is inside the i32. * will reach the place where the value is.
        let baz = &foo;
    }
    // References to functions
    {
        fn print_message(msg: &mut String) {
            msg.push_str(", world!");
            println!("{}", msg);
        }
        let mut foo = String::from("Hello");
        print_message(&mut foo);

        fn print_message_with_ownership(mut msg: String) {
            msg.push_str(", world!");
            println!("{}", msg);
        }
        let bar = String::from("Hello");
        print_message_with_ownership(bar);
    }
    // Copy types
    {
        fn print_message(msg: String) {
            println!("{}", msg);
        }
        let foo = String::from("Hello, world!");
        print_message(foo.clone());
        print_message(foo);
    }
    // Collection types
    {
        // Array has the same size and contains the same type
        let metasyntactic = ["foo", "bar", "baz", "qux", "quux", "corge", "grault"];
        // With same value
        let arr = ["arr"; 12];

        let three_to_five = &metasyntactic[2..5];
        let start_at_two = &metasyntactic[1..];
        let end_at_five = &metasyntactic[..5];
        let everything = &metasyntactic[..];
    }
    // Vectors
    {
        let foo: Vec<String> = Vec::new();
        // or
        let foo: Vec<_> = [9, 0, 10].into(); // Vec<_> let Rust choose the Vec type
                                             // or
        let mut foo = vec![3, 4, 6, 12, 24];
        println!("{}", foo.capacity());
        foo.push(32);
        foo.push(64);

        let three_to_five = &foo[2..5];
        let start_at_two = &foo[1..];
        let end_at_five = &foo[..5];
        let everything = &foo[..];
    }
    // Tuples
    {
        // Can hold many things
        let foo = ("Foo", 6, 12.0);
        let bar = foo.0;
        let (_, _, baz) = foo;
    }
    // Match
    {
        let foo: u8 = 5;
        match foo {
            0 => println!("it's zero"),
            1..=5 => println!("it's between 1 and including 5"),
            6 => println!("it's six"),
            n @ 7 => println!("it's {}", n), // give a name to the value of a match
            _ => println!("It's some other number"),
        }

        // With tuple
        let bar = (255, 0, 0);
        match bar {
            (r, _, _) => println!("it's red"),
        }
    }
}
