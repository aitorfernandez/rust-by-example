// Here we have a pub use for each function we want to bring into the japanese scope
// and always declared the mod's
pub use self::farewells::goodbye;
pub use self::greetings::hello;

// The pub use declaration brings the function into scope at this part
// of our module hierarchy.
// Because we've pub used this inside of our japanese module,
// we now have a crates_and_modules::japanese::hello() function
// and a crates_and_modules::japanese::goodbye() function,
// even though the code for them lives in crates_and_modules::japanese::greetings::hello()
// and phrases::japanese::farewells::goodbye().
// Our internal organization doesn't define our external interface.

mod greetings;

mod farewells;
