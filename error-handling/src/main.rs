use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");
    let f = File::open("hello.txt");

    // match f {
    //     Ok(file) => file,
    //     Err(err) => panic!("Problem opening the file: {:?}", err),
    // };

    // match f {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(f) => f,
    //             Err(err) => panic!("Problem creating the file: {:?}", err),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
                panic!("Problem creating the file: {:?}", err);
            })
        } else {
            panic!("Problem opening the file: {:?}", err);
        }
    });

    //  If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the
    //  Result is the Err variant, unwrap will call the panic! macro for us.
    let _ = File::open("hello.txt").unwrap();

    // expect, which is similar to unwrap, lets us also choose the panic! error message
    let _ = File::open("hello.txt").expect("Failed to open hello.txt");
}
