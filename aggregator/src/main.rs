use aggregator::{
    notify,
    notify_v2 as alert,
    notify_v3 as update,
    Summary, Tweet, NewsArticle, NewsFeed
};

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
    notify(&feed);
    notify(&tweet);
    alert(&feed);

    alert(&String::from("lol"));
    update(&feed);
}