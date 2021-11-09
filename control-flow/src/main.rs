fn main() {
    let condition = true;

    if condition {
        // ..
    } else {
        // ...
    }

    // right side statement
    // ok same time
    let number = if condition { 5 } else { 6 };

    // error different types
    // let number = if condition { 5 } else { "6" };

    // loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining {}", remaining);
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
    println!("End count = {}", count);

    // Returning values from loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

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
