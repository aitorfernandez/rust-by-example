use trait_definition::{self, NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("username"),
        content: String::from("lorem ipsum..."),
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Im a headline"),
        location: String::from("London"),
        content: String::from("lorem ipsum..."),
    };

    println!("New article available: {}", article.summarize());
}
