#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    // Associated functions
    fn area(&self) -> u32 {
        // &self is short for self: &self
        self.w * self.h
    }

    fn w(&self) -> bool {
        self.w > 0
    }

    // Associated functions that aren't methods don't use self, often for constructors
    // call with ::
    fn square(size: u32) -> Rectangle {
        Rectangle { w: size, h: size }
    }
}

// Multiple implement blocks
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }
}

fn main() {
    let rect = Rectangle { w: 30, h: 50 };

    let rect = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    println!("The rectangle has a nonzero with; it is {}", rect.w);
}
