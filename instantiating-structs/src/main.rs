// Structs to structure related data

struct User {
    active: bool,
    email: String,
    sign_in_count: u64,
    username: String,
}

// Structs withhou name
struct Color(i32, i32, i32);

const BLACK: Color = Color(0, 0, 0);

// Unit-Like Structs Without Any Fields
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;

    // Create an instance
    let mut u1 = User {
        active: true,
        email: String::from("foo@gmail.com"),
        sign_in_count: 1,
        username: String::from("foo"),
    };

    // access
    u1.active = false;

    let u2 = User {
        email: String::from("bar@gmail.com"),
        ..u1
    };
}

// or
fn build_user(email: String, username: String) -> User {
    User {
        // Shorthand
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
