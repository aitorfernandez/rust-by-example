// Trait definitions are a way to group method signatures together to define a set of behaviors
// necessary to accomplish some purpose.

pub trait Summary {
    // fn summarize(&self) -> String;
    // or with default behavior
    fn summarize(&self) -> String {
        String::from("Summarize....")
    }

    // can call other methods
    fn summarize_location(&self) -> String {
        self.summarize()
    }
}

// we can implement it on the types
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub content: String,
}

impl Summary for NewsArticle {}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!(
//             "{}, with {} ({})",
//             self.headline, self.content, self.location
//         )
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "{}, with {} ({})",
            self.username, self.content, self.retweet
        )
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// Trait boud
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

// pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// Multiple Trait Bounds
// pub fn notify(item: &(impl Summary + Display) {}
// on generic
// pub fun notify<T: Summary + Display>(item: &T) {}

// Trait Bounds with where Clauses
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

// Returning type that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("username"),
        content: String::from("lorem ipsum..."),
        retweet: false,
    }
}
