#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }

    // pub fn print(self, &self, &mut self, mut self) {
    //     // ...
    //     1. Never give it back unless return it, consumes the object
    //     2. Borrow the object immutably. It means that the method can read from the object, but it cannot modify it
    //     3. It means that the method can read from and modify the object
    //     4. Returns a new object with some modifications
    // }

    pub fn age_up(&mut self, n: i32) {
        self.age += n;
    }

    pub fn dropme(self) {}
}

fn main() {
    let mut p = Person::new("Yo".to_string(), 30);
    p.age_up(3); // needs p as mutable

    // p.dropme(); // can't use p anymore

    println!("{p:?}");

    let a = get_age(&p);

    // p.age_up(2); // Doesn't work a is inmutable or let mut a doesn't work too because not
    // possible change to pointer at the same time, access is control an restricted to avoid data
    // race

    println!("person's age {a}");

    p.age_up(2); // Works because a is not needed it anymore
}

fn get_age(p: &Person) -> &i32 {
    &p.age // This pointer is valis as long as p: &Person lives
}
