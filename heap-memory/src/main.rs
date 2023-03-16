#[derive(Debug)]
pub struct LinkedList<T> {
    pub data: T,
    pub next: Option<Box<LinkedList<T>>>,
}

impl<T: std::ops::AddAssign> LinkedList<T> {
    fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    // Heap memory use when you don't know how big data structure is

    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 4,
            next: None,
        })),
    };

    // use ref with Box type
    if let Some(ref mut v) = ll.next {
        v.add_up(3);
    }

    println!("ll {ll:?}");

    let mut s = "   hello   ".to_string();
    let p = s.trim();
    let p = p.to_string();

    println!("{}", find_string(&p));
}

// asume the str input will be the output, attach the lifetime automatically
// fn find_string<'a>(s: &'a str) -> &'a str {
fn find_string(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        if c == 'l' {
            return &s[i..];
        }
    }
    s
}

// needs 'static for the values live as long as the program runs
fn chose_str(n: i32) -> &'static str {
    match n {
        0 => "A",
        1 => "B",
        _ => "C",
    }
}
