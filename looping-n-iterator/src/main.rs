struct Stepper {
    curr: i32,
    max: i32,
    step: i32,
}

impl Iterator for Stepper {
    // type Item;
    // fn next(&mut self) -> Option<Self::Item>;

    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.max {
            None
        } else {
            let res = self.curr;
            self.curr += self.step;
            Some(res)
        }
    }
}

fn main() {
    let mut n = 0;

    loop {
        n += 1;
        if n > 10 {
            break;
        }
        println!("loop {n}");
    }

    n = 0;
    while n < 10 {
        n += 1;
        println!("while {n}");
    }

    for i in 0..10 {
        println!("for {i}");
    }

    let mut st = Stepper {
        curr: 2,
        step: 3,
        max: 15,
    };

    loop {
        match st.next() {
            Some(v) => println!("Stepper loop {v}"),
            None => break,
        }
    }

    st = Stepper {
        curr: 3,
        step: 4,
        max: 15,
    };

    while let Some(v) = st.next() {
        println!("Stepper while {v}");
    }

    st = Stepper {
        curr: 1,
        step: 2,
        max: 15,
    };

    for i in st {
        println!("Stepper for {i}");
    }
}
