fn main() {
    // Variables and Mutability
    {
        let x; // or let x: i32; // or let x: i32 = 12;
        x = 12;
        println!("Variable and Mutability {}", x);
    }
    // Constants
    {
        // Rust's naming convention for constants is to use all uppercase with underscores between
        // words
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        println!("Constant {}", THREE_HOURS_IN_SECONDS);
    }
    // Shadowing
    {
        let x = 12;
        let x = x + 1;
        println!("Shadowing {}", x);
    }
    // Underscore
    {
        let _ = 12; // or let _ = do_somehting() // throws away the result
        let _x = 12; // avoid the compilar warning, a WIP variable
    }
    // Block
    {
        let x = {
            let y = 5;
            let z = 7;
            y + z // evaluation and return
        };
        println!("Block {}", x);
    }
    // Tuple type
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (_, y, _) = tup; // or let (x, y, z) = tup;
        println!("Tuple type {} or {}", tup.0, y);
    }
    // Functions
    {
        fn plus_one(x: i32) -> i32 {
            x + 1 // return final expresion in the block
        }

        let x = plus_one(11);
        println!("Functions {}", x);
    }
    // Control flow
    {
        let condition = true;
        if condition {
            // ...
        } else {
            // ...
        }
        let _number = if condition { 6 } else { 12 };

        // loop
        let mut count = 0;
        'counting_up: loop {
            let mut remaining = 10;

            loop {
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("Control flow loop {}", count);

        // while
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;
        while index < 5 {
            println!("The value is: {}", a[index]);
            index += 1;
        }
        // for loop
        let a = [10, 20, 30, 40, 50];
        for element in a {
            println!("The value is: {}", element);
        }
        // reverse, 3, 2, 1
        for number in (1..4).rev() {
            println!("The value is: {}", number);
        }
    }
    // Ownership
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    {
        // Each value in Rust has a variable that’s called its owner.
        // There can only be one owner at a time.
        // When the owner goes out of scope, the value will be dropped.
        let x = String::from("hello");
        takes_ownership(x);
        // ... x is not loger valid here

        let y = 5;
        makes_copy(y);
        println!("Use {} afterward", y);

        let z = takes_and_give_back(String::from("hello"));
        println!(
            "Takes and gives back, whick all moves its returns value into z {}",
            z
        );

        fn takes_ownership(s: String) {
            println!("Takes ownership of {}", s);
        }

        fn makes_copy(i: i32) {
            println!("Makes copy of {}", i);
            // Implements Copy
            // All integers
            // Boolean type
            // All the floating point types
            // Character type char
            // Tuples that also contain types that implement Copy (i32, i32) -> Yes, (i32, String) -> No
        }

        fn takes_and_give_back(s: String) -> String {
            s
        }
    }
    // References and Borrowind
    {
        let s = String::from("hello");
        let len = calculate_len(&s); // &s lets us create a reference that refers to the value
        println!("References and Borrowind {}, {}", s, len);

        fn calculate_len(s: &String) -> usize {
            s.len()
        }

        // mutable references
        let mut s = String::from("hello");
        change(&mut s);
        println!("References and Borrowind {}", s);

        fn change(s: &mut String) {
            s.push_str(", world");
        }

        // only one mutable reference to a particular piece of data at a time
        // let mut s = String::from("hello");
        // let r1 = &mut s;
        // let r2 = &mut s;
        // println!("{}, {}", r1, r2);
    }
    // Struct -> "AND"
    {
        struct User {
            active: bool,
            sign_in_count: u64,
            username: String,
        }
        // Initialized using struct literals
        let user1 = User {
            active: true,
            sign_in_count: 1,
            username: String::from("foo"),
        };
        let user2 = User {
            username: String::from("bar"),
            ..user1
        };
        let user3 = User { ..user2 };
        println!(
            "Struct {}, {}, {}",
            user3.active, user3.sign_in_count, user3.username
        );

        #[derive(Debug)] // opt in to print debugging information
        struct Rectangle {
            w: u32,
            h: u32,
        }
        let rec = Rectangle { w: 24, h: 96 };
        println!("Struct {:?}", rec);
    }
    // Method syntax
    {
        #[derive(Debug)]
        struct Rectangle {
            w: u32,
            h: u32,
        }
        impl Rectangle {
            fn area(&self) -> u32 {
                self.w * self.h
            }
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.w > other.w && self.h > other.h
            }
        }
        let rec1 = Rectangle { w: 12, h: 12 };
        let rec2 = Rectangle { w: 6, h: 6 };
        println!(
            "Method syntax for the area of the rectangle is {} and can hold value, {}",
            rec1.area(),
            rec1.can_hold(&rec2)
        );
    }
    // Enum -> "OR"
    {
        enum Message {
            // wide variety of types embedded...
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        impl Message {
            fn call(&self) {
                match self {
                    // Message::Write(s) => println!("Enum ::Write {}", s),
                    // or
                    Self::Write(s) => println!("Enum ::Write {}", s), // Self means the type
                    _ => println!("other variant"),
                    // or
                    // _ => (),
                }
            }
        }
        let m = Message::Write(String::from("hello"));
        m.call();
    }
    // Match
    {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
            // ...
        }
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("Match state quarter from {:?}", state);
                    25
                }
            }
        }
        println!(
            "Match value in cents for Penny {}",
            value_in_cents(Coin::Quarter(UsState::Alaska))
        );
    }
    // Option<T>
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                // Some(i) => Some(i + 1),
                Some(i) => {
                    println!("Option<T> {}", i + 1);
                    Some(i + 1)
                }
            }
        }
        let _ = plus_one(Some(5));
    }
    // Packages, crates and modules
    {
        // https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/crates-and-modules.html
    }
    // Vectors, a heap-allocated array
    {
        let v = vec![100, 32, 5];
        for i in &v {
            println!("Vector {}", i);
        }
        // mutable reference
        let mut v = vec![100, 32, 5];
        for i in &mut v {
            // To change the value that the mutable reference refers to, we have to use the
            // dereference operator (*)
            println!("Vector mutable + 1 {}", *i + 1);
        }
    }
    // Keys with Associated Values in Hash Maps
    {
        use std::collections::HashMap;

        // The type HashMap<K, V> stores a mapping of keys of type K to values of type V
        let mut map = HashMap::new();
        map.insert(String::from("blue"), 12);

        let color = String::from("blue");
        let value = map.get(&color);
        println!("Hash Map value of {:?}", value);

        // for (k, v) in map { ... }
        // borrowing to avoid moving into the for loop and use later
        for (k, v) in &map {
            println!("Hash Map values {}: {}", k, v);
        }
        // Overwriting values
        map.insert(String::from("blue"), 24);
        map.insert(String::from("blue"), 32);

        println!("{:?}", map);
    }
    // Error handling
    {
        let v: Option<i32> = Some(96);
        println!("Error handling {}", v.unwrap());

        // Panics
        // let v: Option<i32> = None;
        // v.unwrap();
        // v.expect("... chose the panic! error message ...")

        // fallback value
        let v: Option<i32> = None;
        println!("Error handling with fallback value {}", v.unwrap_or(108));
        // https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else
        // https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_default

        match read_username_from_file("username.txt") {
            Ok(username) => println!("Error handling username {}", username),
            Err(err) => println!("Error handling error {}", err),
        };

        use std::fs::File;
        use std::io;

        fn read_username_from_file(path: &str) -> Result<String, io::Error> {
            let _ = File::open(path)?; // will return here if error
            Ok("ok".to_string())
        }
    }
    // Generic Types
    {
        fn point<T, U>(x: T, y: U) {
            // ...
        }
        // With trait
        fn point_with_trait<T: PartialOrd + Copy>(x: T) {
            // ...
        }
        // Struct
        struct Point<T, U> {
            x: T,
            y: U,
        }
        // In method definition
        impl<T, U> Point<T, U> {
            fn x(&self) -> &T {
                &self.x
            }
            fn mix<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }
    }
    // Traits
    {
        struct Tweet {
            username: String,
            valid: bool,
        }
        trait Summary {
            fn summarize(&self) -> String;
        }
        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("Tweet by {}, valid {}", self.username, self.valid)
            }
        }
        // Trait as parameter
        // We can define a notify function that calls the summarize method on its item parameter,
        // which is of some type that implements the Summary trait
        // fn notify(item: &impl Summary) {
        // or
        fn notify<T: Summary>(item: &T) {
            println!("Trait summary {}", item.summarize());
        }
        // or
        fn multiple_items<T: Summary>(item1: &T, item2: &T) {
            // ...
        }
        // or
        fn multiple_traits<T: Summary + std::fmt::Display>(item: &T) {
            // ...
        }
        // returning types
        fn returns_summarizable() -> impl Summary {
            Tweet {
                username: String::from("foo"),
                valid: true,
            }
        }
    }
    // References with lifetime
    {}
}
