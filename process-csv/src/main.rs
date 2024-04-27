use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Number {
    #[serde(rename(deserialize = "id_type"))]
    r#type: u32,
    direction: DirectionType,
}

type Numbers = std::collections::HashMap<u32, Number>;

fn process_numbers(n: Number, numbers: &mut Numbers) {
    match n.direction {
        DirectionType::Up => {
            //
        }
        DirectionType::Down => {
            //
        }
    }
}

fn main() {
    // let mut rdr = csv::Reader::from_path("test.csv").expect("Could not read from path");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_path("test.csv")
        .expect("Could not read from path");

    let deserialized_numbers = rdr.deserialize::<Number>();

    let mut numbers = Numbers::new();

    for number in deserialized_numbers {
        println!("{number:?}");
    }

    println!("Hello, world!");
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum DirectionType {
    #[serde(rename(deserialize = "up"))]
    Up,
    #[serde(rename(deserialize = "down"))]
    Down,
}
