// fn main() {
//     let w = 30;
//     let h = 50;

//     println!("The area of the rectangle is {} square pixels.", area(w, h));
// }

// fn area(w: u32, h: u32) -> u32 {
//     w * h
// }

// with tuples
// fn main() {
//     let rect = (30, 50);

//     println!("The area of the rectangle is {} square pixels.", area(rect));
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// with Structs
// struct Rectangle {
//     w: u32,
//     h: u32,
// }

// fn main() {
//     let rect = Rectangle { w: 30, h: 50 };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect)
//     );
// }

// fn area(rect: &Rectangle) -> u32 {
//     rect.w * rect.h
// }

// with Derived Traits

#[derive(Debug)] // make the functionality available for other structs
struct Rectangle {
    w: u32,
    h: u32,
}

fn main() {
    let rect = Rectangle { w: 30, h: 50 };

    println!("The area of the rectangle is {:#?} square pixels.", &rect);
}
