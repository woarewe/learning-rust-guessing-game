use aggregator::{Summary, Tweet, NewsArticle, NewsFeed};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, peope"
        ),
        reply: false,
        retweet: false
    };

    let feed = NewsFeed {
        messages: vec![]
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("{}", feed.summarize());
}