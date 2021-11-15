extern crate crates_and_modules;

// use crates_and_modules::english::farewells;
// use crates_and_modules::english::greetings;
use crates_and_modules::english::{farewells, greetings};
use crates_and_modules::japanese;

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());

    println!("Hello in Japanese: {}", japanese::hello());
    println!("Goodbye in Japanese: {}", japanese::goodbye());
}
